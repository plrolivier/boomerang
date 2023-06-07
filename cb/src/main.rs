/*
 * Example to use libsysforward with a tracer and an executor.
 */
mod tracer;
mod executor;

use std::{
    env,
    process::{ exit },
};

use nix::{
    sys::{
        wait::{ waitpid, WaitStatus, WaitPidFlag},
    },
    unistd::{fork, ForkResult, Pid},
};





fn fork_tracer(program: &str, prog_args: &[String]) -> Pid
{
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child: pid }) => {
            println!("Created Tracer process with PID {}", pid);
            return pid;
        }

        Ok(ForkResult::Child) => {
            // Tracer process
            tracer::start_tracer(program, prog_args);
            exit(0);
            //return Pid::from_raw(0);
        }

        Err(err) => {
            println!("Failed to fork for Tracer process: {}", err);
            exit(1);
            //return Pid::from_raw(0);
        }
    }
}


 fn fork_executor() -> Pid
 {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child: pid }) => {
            println!("Created Executor process with PID {}", pid);
            return pid;
        }

        Ok(ForkResult::Child) => {
            // Execurot process
            executor::start_executor();
            exit(0);
        }

        Err(err) => {
            println!("Failed to fork for Executor process: {}", err);
            exit(1);
        }
    }
}

fn wait_children(tracer_pid: Pid, executor_pid: Pid)
{
    /* 
     * Because the executor does not implement yet any mechanism to be remotly managed,
     * we need to kill it when the tracer is done (TODO).
     */
    for _ in 0..2 {
        match waitpid(None, Some(WaitPidFlag::empty())) {
            Ok(WaitStatus::Exited(_, _)) => {}

            Err(err) => {
                println!("Error occurred while waiting for child processes: {}", err);
            }

            _ => {
                println!("Unexpected result while waiting for child processes");
            }
        }
    }

}


/*
 * Start the tracer and executor 2 different processes, 
 * and wait for them to finish.
 */
 fn main()
 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./cb <program> <arguments>");
        return;
    }

    let program = &args[1];
    let prog_args = &args[2..];


    let executor_pid = fork_executor();
    let tracer_pid = fork_tracer(program, prog_args);

    wait_children(tracer_pid, executor_pid);
}
