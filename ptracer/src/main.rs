/*
 * Example to use libsysforward
 */
use std::{
    env,
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
    tracer_engine::Tracer,
    arch::TargetArch,
};



/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
static TRACER_PORT: u16 = 31000;
static EXECUTOR_PORT: u16 = 31001;



/* Tracer process */

fn run_parent(child: Pid)
{
    let pid = child.as_raw();
    let mut tracer = Tracer::new(pid, TargetArch::X86_64, IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT);

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


fn sync_registers(tracer: &mut Tracer) 
{
    let child = Pid::from_raw(tracer.pid);
    let regs = ptrace::getregs(child).unwrap();
    tracer.sync_registers(regs);
}

fn wait_for_syscall(child: Pid) -> bool
{
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


/* Tracee process */

fn run_child(program: &str, prog_args: &[String])
{
    ptrace::traceme().unwrap();
    Command::new(program)
        .args(prog_args)
        .exec();
    panic!("Error starting {} {:?}", program, prog_args);
}


fn main()
{
    // TODO: add more argument to configure the tracer
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./ptracer <program> <arguments>");
        return;
    }

    let program = &args[1];
    let prog_args = &args[2..];


    println!("[PTRACER] Start tracing...");
    match unsafe { fork() } {

        Ok(ForkResult::Child) => {
            run_child(program, prog_args);
        }

        Ok(ForkResult::Parent {child}) => {
            run_parent(child);
        }

        Err(err) => {
            panic!("[PTRACER] fork() failed: {}", err);
        }
    };

}
