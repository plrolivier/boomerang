/*
 *
 */
use std::{
    os::unix::process::{ CommandExt },
    process::{ self, exit, Child, Command, Stdio },
    sync::{ 
        Arc, Barrier,
        mpsc::{ channel, Sender, Receiver },
    },
    io,
};
use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::{ Signal },
    },
    unistd::{ Pid },
};

use sysforward::{
    sync::{ Event },
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    executor_engine::{ ExecutorEngine, ExecutorCallback },
    operation::Operation,
    targets,
};

use crate::{
    IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT,
};





/*
 *
 */
#[derive(Debug)]
pub struct ExecutingThread {
    tx: Sender<String>,
    rx: Receiver<String>,

    child: Option<Child>,

    // to keep?
    stop: Arc<Event>,
    stopped: Arc<Event>,
}

impl ExecutingThread {

    pub fn new(tx: Sender<String>, rx: Receiver<String>, stop: Arc<Event>, stopped: Arc<Event>) -> Self 
    {
        Self { 
            tx: tx,
            rx: rx,
            child: None,
            stop: stop,
            stopped: stopped,
        }
    }

    pub fn start(&mut self)
    {
        let executor = self.boot_thread().expect("Fail to boot executing thread");

        let executor = self.run_thread(executor).unwrap();

        self.shutdown_thread(executor);
    }

    /*
     * Small code used to setup the executor context
     */
    fn boot_thread(&mut self) -> Result<ExecutorEngine, io::Error>
    {
        println!("***************************************************");
        println!("Executing thread {} booting...", process::id());

        self.spawn_child()?;

        let pid = self.child.as_ref().unwrap().id() as i32;

        let copy_stop = self.stop.clone();
        let copy_stopped = self.stopped.clone();
        let ptrace_op = targets::ptrace::Ptrace{ };
        let regs_op = Box::new(ptrace_op.clone());
        let mem_op = Box::new(ptrace_op);
        let operator = Box::new(Operation{ register: regs_op, memory: mem_op});

        let mut executor = ExecutorEngine::new(TargetArch::X86_64,
                                                               IP_ADDRESS,
                                                               EXECUTOR_PORT,
                                                               TRACER_PORT,
                                                               copy_stop,
                                                               copy_stopped,
                                                               operator,
                                                              );

        /* Show initial memory layout */
        let pid = self.child.as_ref().unwrap().id();
        let mem = read_process_memory_maps(pid);
        print_memory_regions(&mem);

        // Send the PID of the child to the control thread
        self.tx.send(pid.to_string()).unwrap();

        Ok(executor)
    }

    fn spawn_child(&mut self) -> Result<(), io::Error>
    {
        let program = "/bin/true";
        let mut command = Command::new(program);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        unsafe {
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                Ok(())
            });
        }
        self.child = Some(command.spawn().expect("Failed to spawn child process"));

        Ok(())
    }

    fn run_thread(&self, mut executor: ExecutorEngine) -> Result<ExecutorEngine, io::Error>
    {
        // TODO: HOW TO STOP ???
        executor.run();
        Ok(executor)
    }

    /* */
    pub fn shutdown_thread(&mut self, mut executor: ExecutorEngine) -> Result<(), io::Error>
    {
        let pid = self.child.as_ref().unwrap().id();
        println!("Executing thread {} shutdown", pid);
        Ok(())
    }

}
