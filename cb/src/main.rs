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



fn run_tracer()
{
    Command::new("../ptracer/target/debug/ptracer").exec();
    exit(0);
}

fn run_executor()
{
    Command::new("../executor/target/debug/executor").exec();
    exit(0);
}

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
                    run_tracer();
                }

                Err(err) => {
                    println!("Failed to fork for Tracer process: {}", err);
                    exit(1);
                }
            }
        }

        Ok(ForkResult::Child) => {
            // Execurot process
            run_executor();
        }

        Err(err) => {
            println!("Failed to fork for Executor process: {}", err);
            exit(1);
        }
    }
}
