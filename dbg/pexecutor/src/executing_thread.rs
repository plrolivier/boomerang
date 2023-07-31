/*
 *
 */
use core::ffi::c_void;
use std::{
    os::unix::process::CommandExt,
    process::{ self, Child, Command, Stdio },
    sync::{ 
        Arc,
        mpsc::{ Sender, Receiver },
    },
    io,
};
use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::Pid,
};
#[cfg(target_os = "linux")]
use libc;

use sysfwd::{
    sync::Event,
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    executor_engine::{ ExecutorEngine, Invoker },
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
    _rx: Receiver<String>,

    child_pid: Option<i32>,

    stop: Arc<Event>,
    stopped: Arc<Event>,
}

impl ExecutingThread {

    pub fn new(tx: Sender<String>, rx: Receiver<String>, stop: Arc<Event>, stopped: Arc<Event>) -> Self 
    {
        Self { 
            tx: tx,
            _rx: rx,
            child_pid: None,
            stop: stop,
            stopped: stopped,
        }
    }

    pub fn start(&mut self)
    {
        let executor = self.boot_thread().expect("Fail to boot executing thread");

        let executor = self.run_thread(executor).unwrap();

        self.shutdown_thread(executor).unwrap();
    }

    /*
     * Small code used to setup the executor context
     */
    fn boot_thread(&mut self) -> Result<ExecutorEngine, io::Error>
    {
        println!("Executing thread {} booting...", process::id());

        let mut invoker = ExecInvoker::new();
        self.spawn_child(&mut invoker)?;

        let copy_stop = self.stop.clone();
        let copy_stopped = self.stopped.clone();
        let ptrace_op = targets::ptrace::Ptrace{ };
        let regs_op = Box::new(ptrace_op.clone());
        let mem_op = Box::new(ptrace_op);
        let operator = Box::new(Operation{ register: regs_op, memory: mem_op});

        let executor = ExecutorEngine::new(TargetArch::X86_64,
                                                               IP_ADDRESS,
                                                               EXECUTOR_PORT,
                                                               TRACER_PORT,
                                                               copy_stop,
                                                               copy_stopped,
                                                               operator,
                                                               Box::new(invoker),
                                                               self.child_pid.unwrap(),
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
    pub fn shutdown_thread(&mut self, mut _executor: ExecutorEngine) -> Result<(), io::Error>
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
                      _arg7: usize)
                      -> Result<(usize, usize), io::Error>
    {
        /* Setup the register context */        
        let pid = self.child.as_ref().unwrap().id() as i32;
        let pid = Pid::from_raw(pid);
        let mut regs = ptrace::getregs(pid).unwrap();
        let saved_regs = regs.clone();

        // setup syscall instruction somewhere
        let address = 0x555555555000;
        let addr = address as ptrace::AddressType;
        let word = 0x9090050f as *mut c_void;
        let saved_word = ptrace::read(pid, addr).unwrap() as *mut c_void;
        unsafe {
            ptrace::write(pid, addr, word).unwrap();
        }
        regs.rip = address;
        
        // syscall registers:
        // x86_64 specific
        regs.rax = scno as u64;
        regs.orig_rax = scno as u64;
        regs.rdi = arg1 as u64;
        regs.rsi = arg2 as u64;
        regs.rdx = arg3 as u64;
        regs.r10 = arg4 as u64;
        regs.r8  = arg5 as u64;
        regs.r9  = arg6 as u64;
        // nothing on the stack
        ptrace::setregs(pid, regs).unwrap();

        
        // Debug:
        //let regs2 = ptrace::getregs(pid).unwrap();
        //println!("regs before step: {:?}", regs2);
        /* small checks on memory
        let path_addr = arg2 as *mut c_void;
        let mut pathname: Vec<u8> = Vec::new();
        let mut count = 12;
        while count > 0 {
            let word: u32 = ptrace::read(pid, path_addr).unwrap() as u32;
            pathname.extend_from_slice(&word.to_le_bytes());
            count -= 4;
        }
        println!("pathname: {:?}", pathname);
        println!("pathname: {}", std::str::from_utf8(&pathname).unwrap());
        */

        /* Invoke the syscall */
        ptrace::step(pid, None).unwrap();

        match waitpid(pid, None) {
            Err(err) => panic!("Oops something happens when waiting: {}", err),
            Ok(status) => {
                match status {
                    WaitStatus::Stopped(pid, signo) => {
                        match signo {
                            Signal::SIGTRAP => (), // the syscall returned
                            Signal::SIGSEGV => {
                                let regs = ptrace::getregs(pid).unwrap();
                                println!("Tracee {} segfault at {:#x}", pid, regs.rip);
                            },
                            _ => panic!("Tracee {} received signal {} which is not handled", pid, signo),
                        }
                    },
                    WaitStatus::Exited(pid, exit_status) => {
                        panic!("The tracee {} exits with status {}", pid, exit_status);
                    },
                    _ => panic!("WaitStatus not handled"),
                }
            },
        }

        /* Capture what changed */
        let regs = ptrace::getregs(pid).unwrap();
        // Debug:
        //println!("exit regs: {:?}", regs);
        let retval = regs.rax as usize;
        let errno = regs.rdx as usize;

        /* Restore the context (optional) */
        ptrace::setregs(pid, saved_regs).unwrap();
        unsafe {
            ptrace::write(pid, addr, saved_word).unwrap();
        }

        Ok((retval, errno))

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
                // Disable ASLR
                libc::personality(libc::ADDR_NO_RANDOMIZE.try_into().unwrap());
                Ok(())
            });
        }

        let child = command.spawn()?;

        Ok(child)
    }
}