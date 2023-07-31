/*
 * Example to use libsysfwd with ptrace.
 * Works with an executor instance.
 */

mod tracing_thread;


use std::{
    collections::HashMap,
    thread::{ Builder, JoinHandle },
    sync::{ 
        Arc, Barrier,
        mpsc::{ channel, Sender, Receiver },
    },
    io::{self, ErrorKind },
    net::Ipv4Addr,
};

use nix::{
    sys::{
        ptrace,
        signal::Signal
    },
    unistd::Pid,
};

use sysfwd::{
    protocol::control::{ Configuration, ControlChannel },
    tracer::TracerCallback,
};

use crate::tracing_thread::TracingThread;


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
 * A structure used to control tracing thread.
 */
struct ThreadCtrl {
    handler: JoinHandle<()>,
    barrier: Arc<Barrier>,
    _tx: Sender<String>,
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
        
        let thread_ctrl = ThreadCtrl {
            handler: handler,
            barrier: barrier_copy,
            _tx: tx_ctrl,
            rx: rx_thread
        };

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
