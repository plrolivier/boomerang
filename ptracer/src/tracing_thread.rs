/*
 * The code relevant to the tracing of a single thread.
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

#[cfg(target_os = "linux")]
use libc;

use sysforward::{
    arch::TargetArch,
    tracer::{ TracerEngine },
    operation::Operation,
    targets,
    memory::{ read_process_memory_maps, print_memory_regions },
};

use filters::{
    ForwardFileRule,
};

use crate::{
    IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT,
};



/*
 * Represent a thread tracing the execution of a child thread.
 */
//#[derive(Clone, Debug)]
pub struct TracingThread {
    pub boot_barrier: Arc<Barrier>,
    tx: Sender<String>,
    rx: Receiver<String>,

    program: String,
    prog_args: Vec<String>,
    tracee: Option<Child>,
    //tracer: Option<TracerEngine>,

    //use_pkexec: bool,
 }


impl TracingThread {

    pub fn new(program: String, prog_args: Vec<String>, tx: Sender<String>, rx: Receiver<String>, barrier: Arc<Barrier>) -> Self 
    {
        TracingThread { 
            boot_barrier: barrier,
            tx: tx,
            rx: rx,
            program: program,
            prog_args: prog_args,
            tracee: None,
            //tracer: None,
            //use_pkexec: true,
        }
    }
    
    pub fn start(&mut self)
    {
        let tracer = self.boot_thread().expect("Fail to setup tracing thread");

        // Wait for the signal to start the tracee execution and syscall tracing from the control thread.
        self.boot_barrier.wait();

        let tracer = self.run_thread(tracer).unwrap();

        self.shutdown_thread(tracer).expect("Fail to properly clean tracing thread");
    }

    /*
     * Small code used to setup the tracing context.
     */
    fn boot_thread(&mut self) -> Result<TracerEngine, io::Error>
    {
        println!("***************************************************");
        println!("Tracing thread {} booting...", process::id());

        /* Setup the tracee */
        self.spawn_tracee(self.program.clone(), self.prog_args.clone())?; 
        let pid = self.tracee.as_ref().unwrap().id() as i32;

        let mem = read_process_memory_maps(pid as u32);
        print_memory_regions(&mem);

        /* Setup the tracer */
        let ptrace_op = targets::ptrace::Ptrace{ };
        let regs_op = Box::new(ptrace_op.clone());
        let mem_op = Box::new(ptrace_op);
        let operator = Box::new(Operation{ register: regs_op, memory: mem_op });

        let mut tracer = TracerEngine::new(pid,
                                                         TargetArch::X86_64,
                                                         IP_ADDRESS,
                                                         TRACER_PORT,
                                                         EXECUTOR_PORT,
                                                         operator,
                                                        );

        /* Load filters */
        let rule = Box::new(ForwardFileRule::new(String::from("/dev/kbuf")));
        tracer.load_rule(0, rule);
        
        // Send the PID of the tracee to the control thread
        self.tx.send(pid.to_string()).unwrap();

        Ok(tracer)
    }

    /*
     * Spawn the process where the tracee program will live.
     * Use PTRACE_TRACEME and waits for the tracer thread to initialize.
     */
    fn spawn_tracee(&mut self, mut program: String, mut prog_args: Vec<String>) -> Result<(), io::Error>
    {
        /*
        if self.use_pkexec {
            prog_args.insert(0, program);
            program = String::from_str("pkexec").unwrap();
        }
        */

        println!("Spawnning {} {:?}", program, prog_args);
        let mut command = Command::new(program);
        command.args(prog_args);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        unsafe {
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                // Disable ASLR for the program
                libc::personality(libc::ADDR_NO_RANDOMIZE.try_into().unwrap());
                Ok(())
            });
        }
        self.tracee = Some(command.spawn().expect("Failed to spawn child process"));

        /* 
        let status: std::process::ExitStatus = self.tracee.wait().unwrap();
        println!("Status is {:?}", status);
        println!("success: {:?}", status.success());
        println!("code: {:?}", status.code());
        */

        Ok(())
    }

    fn _attach_tracee(&mut self, _pid: u32) -> Result<(), io::Error>
    {
        /* TODO */
        panic!("Not implemented");
    }

    fn shutdown_thread(&mut self, mut tracer: TracerEngine) -> Result<(), io::Error>
    {
        println!("Thread tracing process {} shutdown", self.tracee.as_ref().unwrap().id());
        //let status = self.tracee.as_mut().unwrap().wait().expect("Not running");
        //println!("Tracee exits with status {}", status.code().unwrap());
        tracer.shutdown().unwrap();
        Ok(())
    }

    fn run_thread(&mut self, mut tracer: TracerEngine) -> Result<TracerEngine, io::Error>
    {
        let pid = Pid::from_raw(self.tracee.as_ref().unwrap().id() as i32);
        /*
         * The main loop
         */
        loop {
            self.restart_syscall(pid).unwrap();

            match self.wait_for_syscall(pid) {
                Err(()) => break,

                Ok(pid) => { 
                    self.sync_registers(pid, &mut tracer)?;
                    tracer.trace()?;
                },
            }
        }
        Ok(tracer)
    }

    fn sync_registers(&self, pid: Pid, tracer: &mut TracerEngine) -> Result<(), io::Error>
    {
        let regs: nix::libc::user_regs_struct = ptrace::getregs(pid)?;
        tracer.sync_registers(regs);
        Ok(())
    }

    fn restart_syscall(&self, pid: Pid) -> Result<Pid, ()>
    {
        // Continue execution
        //ptrace::syscall(pid, None).unwrap();
        match ptrace::syscall(pid, None) {
            Ok(()) => { /* continue */ },
            /*
            Err(ref err) if err.kind() == nix::errno::Errno::ESRCH => {
                println!("ESRCH: No such process: {:?}", err);
                return false;
            }
            */
            Err(err) => {
                panic!("Fail to restart tracee: {:?}", err);
            }
        }
        Ok(pid)
    }

    fn wait_for_syscall(&self, pid: Pid) -> Result<Pid, ()>
    {
        match waitpid(pid, None) {
            Err(err) => {
                panic!("Oops something happens when waiting: {}", err);
            },

            Ok(status) => {
                match status {
                    WaitStatus::Stopped(pid, signo) => {
                        match signo {
                            Signal::SIGTRAP => {
                                return Ok(pid);
                            },
                            Signal::SIGSEGV => {
                                let regs = ptrace::getregs(pid).unwrap();
                                println!("Tracee {} segfault at {:#x}", pid, regs.rip);
                                return Err(());
                            },
                            // TODO: add support for other signals
                            _ => {
                                panic!("Tracee {} received signal {} which is not handled", pid, signo);
                            },
                        }
                    },
                    WaitStatus::Exited(pid, exit_status) => {
                        println!("The tracee {} exits with status {}", pid, exit_status);
                        return Err(());
                    },
                    // TODO: add support for other WaitStatus
                    _ => {
                        panic!("WaitStatus not handled");
                    },
                }
            },
        }
    }
}
