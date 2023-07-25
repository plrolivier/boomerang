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
    executor_engine::{ ExecutorEngine, Invoker },
    syscall::{ RawSyscall },
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

    child_pid: Option<i32>,

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
            child_pid: None,
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

        let mut invoker = ExecInvoker::new();
        self.spawn_child(&mut invoker)?;

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
                                                               Box::new(invoker),
                                                              );

        /* Show initial memory layout */
        let pid = self.child_pid.unwrap() as u32;
        let mem = read_process_memory_maps(pid);
        print_memory_regions(&mem);

        // Send the PID of the child to the control thread
        self.tx.send(pid.to_string()).unwrap();

        Ok(executor)
    }

    fn spawn_child(&mut self, invoker: &mut ExecInvoker) -> Result<(), io::Error>
    {
        let child = invoker.invoke_new_process().expect("Failed to spawn child process");
        let pid = child.id() as i32;

        invoker.child = Some(child);
        self.child_pid = Some(pid);

        Ok(())
    }

    fn run_thread(&self, mut executor: ExecutorEngine) -> Result<ExecutorEngine, io::Error>
    {
        executor.run();
        Ok(executor)
    }

    /* */
    pub fn shutdown_thread(&mut self, mut executor: ExecutorEngine) -> Result<(), io::Error>
    {
        println!("Executing thread {} shutdown", self.child_pid.unwrap());

        // ...

        Ok(())
    }

}



struct ExecInvoker {
    child: Option<Child>,
 }

impl ExecInvoker {
    fn new() -> Self
    {
        Self { child: None }
    }
}

impl Invoker for ExecInvoker {

    fn invoke_syscall(&self, scno: usize, arg1:usize, arg2: usize,
                      arg3: usize, arg4: usize, arg5: usize, arg6: usize,
                      arg7: usize) -> Result<RawSyscall, io::Error>
    {
        
        // Continue here

        /* Setup the context */        
        // memory
        // registers

        /* Invoke the syscall */


        /* Capture what changed */


        let raw = RawSyscall::new();
        Ok(raw)
    }


    fn invoke_new_process(&self) -> Result<Child, io::Error>
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

        let child = command.spawn()?;

        Ok(child)
    }
}