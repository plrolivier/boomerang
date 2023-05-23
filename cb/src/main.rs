/*
 * Example to use libsysforward with a tracer and an executor.
 */
use core::ffi::c_void;
use std::{
    os::unix::process::CommandExt,
    process::{exit, Command},
    collections::VecDeque,
    thread,
    time,
};
use nix::{
    sys::{
        ptrace,
        wait::{wait, waitpid, WaitStatus, WaitPidFlag},
        signal::Signal,
        signal::kill,
    },
    unistd::{fork, ForkResult, Pid},
};

use sysforward::{
    arch::TargetArch,
    tracer_engine::Tracer,
    executor_engine::Executor,
};




/*
 * Tracer code
 */

fn ptracer() {

    println!("[ptracer] Start tracing...");
    //thread::sleep(std::time::Duration::from_secs(1));

    match unsafe { fork() } {

        Ok(ForkResult::Child) => {
            run_child();
        }

        Ok(ForkResult::Parent {child}) => {
            run_parent(child);
        }

        Err(err) => {
            panic!("[ptracer] fork() failed: {}", err);
        }
    };
}

fn run_child() {
    ptrace::traceme().unwrap();
    Command::new("ls").exec();
    exit(0);
}

fn run_parent(child: Pid) {

    let pid = child.as_raw();
    let mut tracer = Tracer::new(pid, TargetArch::X86_64);

    wait().unwrap();    // exit syscall

    /*
     * The main loop of the program.
     */
    loop {
        match wait_for_syscall(child) {
            false => break,
            true => (),
        }
        sync_registers(&mut tracer);
        tracer.trace();
    }
}


fn sync_registers(tracer: &mut Tracer) {

    let child = Pid::from_raw(tracer.pid);
    let regs = ptrace::getregs(child).unwrap();
    tracer.sync_registers(regs);
}

fn wait_for_syscall(child: Pid) -> bool {

    ptrace::syscall(child, None).unwrap();

    match wait() {
        Err(err) => {
            panic!("Oops something happens when waiting: {}", err);
        },
        Ok(status) => {
            match status {
                WaitStatus::Stopped(child, signo) => {
                    match signo {
                        Signal::SIGTRAP => {
                            return true;
                        },
                        Signal::SIGSEGV => {
                            let regs = ptrace::getregs(child).unwrap();
                            println!("Tracee {} segfault at {:#x}", child, regs.rip);
                            return false;
                        },
                        // TODO: add support for other signals
                        _ => {
                            println!("Tracee {} received signal {} which is not handled", child, signo);
                            return false;
                        },
                    }
                },
                WaitStatus::Exited(child, exit_status) => {
                    println!("The tracee {} exits with status {}", child, exit_status);
                    return false;
                },
                // TODO: add support for other WaitStatus
                _ => {
                    panic!("WaitStatus not handled");
                },
            }
        },
    }
}



/*
 * Executor code
 */

fn executor() {
    println!("[executor] Start listening...");
    //thread::sleep(std::time::Duration::from_secs(2));

    let mut executor = Executor::new(TargetArch::X86_64);

    executor.run();
}



/*
 * Parent code
 */
fn wait_children() {
    for _ in 0..2 {
        match waitpid(None, Some(WaitPidFlag::empty())) {
            Ok(WaitStatus::Exited(_, _)) => {}
            Err(err) => {
                println!("Error occurred while waiting for child process: {}", err);
            }
            _ => {
                println!("Unexpected result while waiting for child process");
            }
        }
    }
}

/*
 * Start the tracer and executor 2 different processes, 
 * and wait for them to finish.
 */
 fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child: executor_pid }) => {
            println!("Created Executor process with PID {}", executor_pid);

            match unsafe { fork() } {
                Ok(ForkResult::Parent { child: tracer_pid }) => {
                    println!("Created Tracer process with PID {}", tracer_pid);
                    // Wait for both processes to finish
                    wait_children();
                }

                Ok(ForkResult::Child) => {
                    // Tracer process
                    ptracer();
                    exit(0);
                }

                Err(err) => {
                    println!("Failed to fork for Tracer process: {}", err);
                    exit(1);
                }
            }
        }

        Ok(ForkResult::Child) => {
            // Execurot process
            executor();
            exit(0);
        }

        Err(err) => {
            println!("Failed to fork for Executor process: {}", err);
            exit(1);
        }
    }
}



