/*
 *
 */
mod executing_thread;


use std::{
    collections::HashMap,
    thread::{ Builder, JoinHandle },
    sync::{ 
        Arc,
        mpsc::{ channel, Sender, Receiver },
    },
    net::Ipv4Addr,
    io::{self, ErrorKind },
};

use nix::unistd::Pid;

use sysfwd::{
    arch::TargetArch,
    sync::Event,
    protocol::control::{ Configuration, ControlChannel },
    executor::ExecutorCallback,
};

use crate::executing_thread::ExecutingThread;


/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
//static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 32000;
static EXECUTOR_PORT: u16 = 32001;



/*
 * The debugger is the high-level structure which manage the executing threads and the connection with the python commands.
 */
struct ExecDebugger {
    control_channel: ControlChannel,
}

impl ExecDebugger {

    pub fn new(target_arch: TargetArch) -> Self
    {
        Self {
            control_channel: ControlChannel::new(Configuration::Executor, None, Some(Box::new(ExecDebuggerCallback::new(&target_arch)))),
        }
    }

    pub fn run(&mut self)
    {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 31001;

        self.control_channel.listen(ip, port);
    }
}


/*
 * A structure used to control an executing thread.
 */
struct ThreadCtrl {
    handler: JoinHandle<()>,
    _tx: Sender<String>,
    rx: Receiver<String>,
    stop: Arc<Event>,
    stopped: Arc<Event>,
}


struct ExecDebuggerCallback {
    thread_map: HashMap<Pid, ThreadCtrl>,
    target_arch: Arc<TargetArch>,
}

impl ExecDebuggerCallback {

    pub fn new(target_arch: &TargetArch) -> Self
    {
        Self {
            thread_map: HashMap::new(),
            target_arch: Arc::new(*target_arch),
        }
    }
}

impl ExecutorCallback for ExecDebuggerCallback {

    fn spawn_process(&mut self, _program: &str, _prog_args: &[&str]) -> Result<Pid, io::Error>
    {
        println!("Creating new executing thread...");

        let (tx_ctrl, rx_ctrl) = channel();
        let (tx_thread, rx_thread) = channel();
        let stop = Arc::new(Event::new());
        let stop_clone = stop.clone();
        let stopped = Arc::new(Event::new());
        let stopped_clone = stopped.clone();

        let arch = Arc::clone(&self.target_arch);
        let mut executing_thread = ExecutingThread::new(arch, tx_thread, rx_ctrl, stop, stopped);

        /* Creat thread and start it */
        let builder = Builder::new();
        let handler = builder.spawn(move ||
            executing_thread.start()
        ).unwrap();

        let thread_ctrl = ThreadCtrl { 
            handler: handler,
            _tx: tx_ctrl,
            rx: rx_thread,
            stop: stop_clone,
            stopped: stopped_clone,
        };

        let pid = thread_ctrl.rx.recv().unwrap();
        let pid = pid.parse().unwrap();
        let pid = Pid::from_raw(pid);
        self.thread_map.insert(pid, thread_ctrl);
        
        Ok(pid)
    }

    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Kill process {:?} *", pid);
        
        // TODO
        match self.thread_map.remove(&pid) {
            Some(thread) => {

                println!("stopping..");
                if ! thread.stopped.is_set() {
                    thread.stop.set();
                    thread.stopped.wait();
                }

                match thread.handler.join() {
                    Ok(()) => { },
                    Err(err) => {
                        println!("Couldn't joind the thread {}: {:?}", pid, err);
                        return Err(io::Error::new(ErrorKind::Other, "Couldn't join the thread"))
                    }
                }
            },

            None => {
                // nix error: ESRC
                println!("Error: No such process: {}", pid);
                return Err(io::Error::new(ErrorKind::Other, "No such pid"))
            }
        }
        Ok(())
    }
}



fn main()
{
    /* TODO: add more argument to configure the executor:
     *  - etc.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./executor <>");
        return;
    }
    */

    let mut dbg = ExecDebugger::new(TargetArch::X86_64);

    // Listen for incomming connection and order from a trace dgb
    println!("[EXECUTOR] Start debugger...");
    dbg.run();

    println!("[EXECUTOR] Stop debugger.");
}