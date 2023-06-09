/*
 * Example to use libsysforward
 */
use std::{
    collections::{ HashMap },
    thread::{ Builder, JoinHandle },
    os::unix::process::CommandExt,
    process::{ exit, Child, Command },
    sync::{ Arc, Barrier }, hash::Hash,
    io::{self, prelude::*, BufReader, BufWriter },
    net::{ TcpStream },
};
use std::fmt::Error;

use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::{ Pid },
};

use sysforward::{
    tracer_engine::Tracer,
    arch::TargetArch,
};

use crate::{ IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT };



/*
 * The debugger is the high-level structure which manage the tracing threads and connection with the executor.
 */
struct TraceDebugger {
    handler_map: HashMap<Pid, Option<JoinHandle<()>>>,
    thread_map: HashMap<Pid, TracingThread>,

    reader: Option<BufReader<TcpStream>>,
    writer: Option<BufWriter<TcpStream>>,
}

impl TraceDebugger {

    pub fn new() -> Self
    {
        Self {
            handler_map: HashMap::new(),
            thread_map: HashMap::new(),
            reader: None,
            writer: None,
        }

    }

    pub fn init(&mut self)
    {
        // TODO

        self.connect();
    }

    fn connect(&mut self)
    {
        let stream = TcpStream::connect("127.0.0.1:31000").unwrap();

        self.reader = Some(BufReader::new(stream.try_clone().unwrap()));
        self.writer = Some(BufWriter::new(stream));
    }

    fn check_connected(&self) -> bool
    {
        if self.reader.is_none() || self.writer.is_none() {
            return false;
        } else {
            return true;
        }
    }

    pub fn spawn(&mut self, program: &str, prog_args: &[String]) -> Result<Pid, io::Error>
    {
        // Verify the connection with executor
        if ! self.check_connected() {
            return Err(io::Error::new(io::ErrorKind::Other, "Not connected with executor"));
        }

        // Spawn the child
        println!("[TRACER] Spawn {} {:?}", program, prog_args);
        let child: Child = unsafe {
            let mut command = Command::new(program);
            command.args(prog_args);
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                Ok(())
            });

            command.spawn().expect("Failed to spawn child process")
        };

        let pid = Pid::from_raw(child.id() as i32);

        // Wait for first syscall
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => { /* ??? */ },
            _ => panic!("WaitStatus not handled"),
        };

        // Create the tracing thread
        let boot_barrier = Arc::new(Barrier::new(2));

        let tracing_thread = TracingThread { boot_barrier };
        let copy_tracing_thread = tracing_thread.clone();
        self.thread_map.insert(pid, tracing_thread);

        let builder = Builder::new().name(child.id().to_string());
        let handler = builder.spawn(move ||
            copy_tracing_thread.boot_thread(child)
        ).unwrap();
        self.handler_map.insert(pid, Some(handler));

        // Notify the executor

        Ok(pid)
    }

    
    pub fn start_tracing_thread(&self, pid: Pid)
    {
        let thread = self.thread_map.get(&pid).unwrap();
        thread.boot_barrier.wait();
    }

    pub fn join_tracing_thread(&mut self, pid: Pid)
    {
        if let Some(handler) = self.handler_map.get_mut(&pid) {
            if let Some(thread) = handler.take() {
                thread.join().unwrap();
            }
        }
    }

}


/*
 * Represent a thread tracing the execution of a child thread.
 */
#[derive(Clone, Debug)]
struct TracingThread {
    boot_barrier: Arc<Barrier>,
 }


impl TracingThread {

    /*
     * Small code given to a newly spawn tracing thread for waiting to start tracing.
     */
    fn boot_thread(&self, tracee: Child)
    {
        // Setup the tracer
        let tracer = Tracer::new(tracee.id() as i32, TargetArch::X86_64, IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT);
        
        // Wait for the main thread to start tracing
        self.boot_barrier.wait();
        self.run_thread(tracee, tracer);
    }

    fn run_thread(&self, tracee: Child, mut tracer: Tracer)
    {
        let pid = Pid::from_raw(tracee.id() as i32);
        /*
         * The main tracing
         */
        loop {
            match self.wait_for_syscall(pid) {
                false => break,
                true => (),
            }
            
            self.sync_registers(pid, &mut tracer);
            
            tracer.trace();
        }
    }

    fn sync_registers(&self, pid: Pid, tracer: &mut Tracer) 
    {
        let regs: nix::libc::user_regs_struct = ptrace::getregs(pid).unwrap();
        tracer.sync_registers(regs);
    }

    fn wait_for_syscall(&self, pid: Pid) -> bool
    {
        // Continue execution
        ptrace::syscall(pid, None).unwrap();

        match waitpid(pid, None) {
            Err(err) => {
                panic!("Oops something happens when waiting: {}", err);
            },

            Ok(status) => {
                match status {
                    WaitStatus::Stopped(pid, signo) => {
                        match signo {
                            Signal::SIGTRAP => {
                                return true;
                            },
                            Signal::SIGSEGV => {
                                let regs = ptrace::getregs(pid).unwrap();
                                println!("Tracee {} segfault at {:#x}", pid, regs.rip);
                                return false;
                            },
                            // TODO: add support for other signals
                            _ => {
                                println!("Tracee {} received signal {} which is not handled", pid, signo);
                                return false;
                            },
                        }
                    },
                    WaitStatus::Exited(pid, exit_status) => {
                        println!("The tracee {} exits with status {}", pid, exit_status);
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
}


pub fn start_tracer(program: &str, prog_args: &[String])
{
    let mut dbg = TraceDebugger::new();

    // Connect to the executor
    //dbg.connect();

    // Create the process to trace
    let pid: Pid = dbg.spawn(program, prog_args).unwrap();

    // Start tracing system calls
    println!("[TRACER] Start tracing...");
    dbg.start_tracing_thread(pid);

    dbg.join_tracing_thread(pid);
    println!("[TRACER] Finish tracing.");
}
