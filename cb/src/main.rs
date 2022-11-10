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
        wait::{wait, WaitStatus},
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

    let mut executor = Executor::new(TargetArch::X86_64);

    executor.listen();
}





fn main() {
    /*
     * Start the executor and tracer in 2 different threads, and wait them to finish.
     */

    let executor_handler = thread::spawn(|| { executor(); });

    let tracer_handler = thread::spawn(|| { ptracer(); });

    tracer_handler.join().unwrap();
    /* Note: the executor should stop after its TCP connection is closed */
    executor_handler.join().unwrap(); 
}

