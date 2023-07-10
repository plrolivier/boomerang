/*
 * Example to use libsysforward with ptrace.
 * Works with an executor instance.
 */
use std::{
    collections::{ HashMap },
    thread::{ self, Builder, JoinHandle },
    os::unix::process::{ CommandExt },
    process::{ self, exit, Child, Command, Stdio },
    sync::{ 
        Arc, Barrier,
        mpsc::{ channel, Sender, Receiver },
    },
    io::{self, prelude::*, BufReader, BufWriter, ErrorKind },
    net::{ Ipv4Addr }, str::FromStr,
};

use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::{ Signal, kill },
    },
    unistd::{ Pid },
};

use sysforward::{
    arch::TargetArch,
    protocol::control::{ Configuration, ControlChannel },
    tracer_engine::{ TracerCallback, TracerEngine },
};



/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
//static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 32000;
static EXECUTOR_PORT: u16 = 32001;



/*
 * The debugger is the high-level structure which manage the tracing threads and connection with the executor.
 */
struct TraceDebugger {
    control_channel: ControlChannel,
}


impl TraceDebugger {

    pub fn new() -> Self
    {
        // TODO: configure with ptrace?

        /* HERE !!!
         * The idea is to be able to call from the control channel some functions from the debugger,
         * such as spawn_process, kill_process, start_tracing, read_mem, write_regs, set_breakpoint, etc.
         * 
         * For that, we need a callback mechanisms to "register" or refer to the right function within the 
         * control_channel object.
         * 
         * We could use:
         *      1. function pointers
         *      2. callback with closure
         *      3. Trait
         *      4. Rc<RefCell<>>
         * 
         */
        Self {
            control_channel: ControlChannel::new(Configuration::Tracer, Some(Box::new(TraceDebuggerCallback::new())), None)
        }
    }

    pub fn run(&mut self)
    {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 31000;

        //self.control_channel.connect(ip, port).unwrap();

        self.control_channel.listen(ip, port);
    }
}

/*
impl Default for TraceDebugger {
    fn default() -> Self {
        Self::new()
    }
}
*/


struct ThreadCtrl {
    handler: JoinHandle<()>,
    barrier: Arc<Barrier>,
    tx: Sender<String>,
    rx: Receiver<String>,

}

struct TraceDebuggerCallback {
    thread_map: HashMap<Pid, ThreadCtrl>,
}

impl TraceDebuggerCallback {
    pub fn new() -> Self {
        Self {
            thread_map: HashMap::new(),
        }
    }

}

impl TracerCallback for TraceDebuggerCallback {

    fn spawn_process(&mut self, program: String, prog_args: Vec<String>) -> Result<Pid, io::Error>
    {
        println!("Creating new tracing thread...");

        let (tx_ctrl, rx_ctrl) = channel();
        let (tx_thread, rx_thread) = channel();
        let boot_barrier = Arc::new(Barrier::new(2));
        let barrier_copy = boot_barrier.clone();
        let mut tracing_thread = TracingThread::new(program, prog_args, tx_thread, rx_ctrl, boot_barrier);

        /* Create thread and start it */
        let builder = Builder::new();
        let handler = builder.spawn(move ||
            tracing_thread.start()
        ).unwrap();
        
        let thread_ctrl = ThreadCtrl { handler: handler, barrier: barrier_copy, tx: tx_ctrl, rx: rx_thread };

        let pid = thread_ctrl.rx.recv().unwrap();
        let pid = pid.parse().unwrap();
        let pid = Pid::from_raw(pid);
        self.thread_map.insert(pid, thread_ctrl);

        // Notify the executor
        //self.notify_new_process();

        Ok(pid)
    }

    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Kill process {:?} *", pid);
        
        match self.thread_map.remove(&pid) {
            Some(thread) => {
                println!("killing...");
                // TODO: Instead, ask the tracing thread to kill the process using Child.kill()?
                // TODO: Check the process still lives... otherwise ESRCH...
                //ptrace::kill(pid).unwrap(); // panic when ESRCH !
                match ptrace::kill(pid) {
                    Ok(()) => { },
                    Err(err) => {  
                        // ESRCH ?
                        println!("Couldn't kill process: {}", err);
                    },
                }

                println!("joining...");
                //thread.handler.join().unwrap();
                match thread.handler.join() {
                    Ok(()) => { },
                    Err(err) => {
                        println!("Couldn't joind the thread {}: {:?}", pid, err);
                        return Err(io::Error::new(ErrorKind::Other, "Couldn't join the thread"))
                    }
                }
                println!("kill command finished");
            },

            None => {
                // nix error: ESRC
                println!("Error: No such process: {}", pid);
                return Err(io::Error::new(ErrorKind::Other, "No such pid"))
            }
        }
        Ok(())
    }

    fn start_tracing(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Trace process {:?} *", pid);

        match self.thread_map.get_mut(&pid) {
            Some(thread) => {
                println!("Waiting on boot barrier for {}", pid);
                thread.barrier.wait();
            },

            None => {
                // nix error: ESRC
                println!("Error: No such running process: {}", pid);
                return Err(io::Error::new(ErrorKind::Other, "No such running process"))
            }
        }
        Ok(())
    }

    fn cont_tracing(&mut self, pid: Pid, signal: Option<Signal>) -> Result<(), io::Error>
    {
        println!("* Continue process {:?} with {:?} *", pid, signal);
        match signal {
            Some(signal) => {
                ptrace::cont(pid, signal).unwrap(); 
            },
            None => {
                ptrace::cont(pid, None).unwrap();
            },
        }
        Ok(())
    }

    fn stop_tracing(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Stop process {:?} *", pid);
        //ptrace::
        // TODO: which signal use GDB ?
        Ok(())
    }





    /*
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
        self.notify_new_process();

        Ok(pid)
    }

    fn notify_new_process(&self)
    {
        // TODO: the port depends on how many thread has been spawn
        let payload = control::NewProcessRequestPayload { 
            address_ipv4: IP_ADDRESS,
            tracer_port: TRACER_PORT,
            executor_port: EXECUTOR_PORT,
        };

        let payload = serde_json::to_string(&payload).unwrap();

        let message = control::Message {
            command: control::Command::NewProcess,
            payload: payload,
        };

        let mut data = serde_json::to_string(&message).unwrap();
        data.push_str("\n");
        println!("[TRACER] Send message: {}", data);

        let _ret = self.writer.expect("No TcpStream Writer found").write(data.as_bytes());
        self.writer.expect("No TcpStream Writer found").flush();
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
    */

}

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

        /* Setup the tracer */
        let pid = self.tracee.as_ref().unwrap().id() as i32;
        let tracer = TracerEngine::new(pid,
                                                     TargetArch::X86_64,
                                                     IP_ADDRESS,
                                                     TRACER_PORT,
                                                     EXECUTOR_PORT,
                                                    );
        
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



fn main()
{
    /* TODO: add more argument to configure the tracer:
     *  - program to trace with its arguments
     *  - option on which port to listen
     *  - options for what to trace
     *  - architecture?
     *  - etc.
     * 
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./ptracer <program> <arguments>");
        return;
    }

    let program = &args[1];
    let prog_args = &args[2..];
     */

    let mut dbg = TraceDebugger::new();


    // Start tracing system calls
    println!("[TRACER] Start debugger...");
    dbg.run();

    println!("[TRACER] Stop debugger.");

}
