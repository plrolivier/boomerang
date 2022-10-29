use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::collections::VecDeque;

use core::ffi::c_void;

use nix::sys::ptrace;
use nix::sys::wait::{wait, WaitStatus};
use nix::sys::signal::Signal;
use nix::unistd::{fork, ForkResult, Pid};

use sysforward::tracer_engine::tracer::Tracer;
use sysforward::arch::TargetArch;



fn main() {

    match unsafe {fork()} {

        Ok(ForkResult::Child) => {
            run_child();
        }

        Ok(ForkResult::Parent {child}) => {
            run_parent(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    };
}


fn run_child() {
    ptrace::traceme().unwrap();
    Command::new("ls").exec();
    exit(0);
}

fn run_parent(child: Pid) {

    let mut tracer = Tracer::new(child, TargetArch::X86_64);

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

    let regs = ptrace::getregs(tracer.pid).unwrap();
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


