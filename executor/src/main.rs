/*
 *
 */
use std::{
    collections::{ HashMap },
    os::unix::process::{ CommandExt },
    process::{ exit, Child, Command },
    thread::{ Builder, JoinHandle },
    sync::{ Arc, Barrier },
    net::{ Ipv4Addr },
    io,
};

use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::{ Pid }, Error,
};

use sysforward::{
    sync::{ Event },
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    protocol::control::{ Configuration, ControlChannel },
    executor_engine::{ ExecutorEngine, ExecutorCallback },
};


/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
//static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 32000;
static EXECUTOR_PORT: u16 = 32001;



/*
 *
 */
struct ExecDebugger {
    control_channel: ControlChannel,
}

impl ExecDebugger {

    pub fn new() -> Self
    {
        // TODO: configure with ptrace?
        Self {
            control_channel: ControlChannel::new(Configuration::Executor, None, Some(Box::new(ExecDebuggerCallback::new()))),
        }

    }

    pub fn run(&mut self)
    {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 31001;

        //self.control_channel.connect(ip, port).unwrap();

        self.control_channel.listen(ip, port);
    }
}

struct ExecDebuggerCallback {
    handler_map: HashMap<Pid, Option<JoinHandle<()>>>,
    thread_map: HashMap<Pid, ExecThread>,
}

impl ExecDebuggerCallback {

    pub fn new() -> Self
    {
        Self {
            handler_map: HashMap::new(),
            thread_map: HashMap::new(),
        }

    }
}

impl ExecutorCallback for ExecDebuggerCallback {

    fn spawn_process(&mut self, program: &str, prog_args: &[&str]) -> Result<Pid, io::Error>
    {
        println!("* Spawn process: {} {:?} *", program, prog_args);
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

        let executing_thread = ExecThread::new(boot_barrier);
        let copy_executing_thread = executing_thread.clone();
        self.thread_map.insert(pid, executing_thread);

        let builder = Builder::new().name(child.id().to_string());
        let handler = builder.spawn(move ||
            copy_executing_thread.boot_thread(child)
        ).unwrap();
        self.handler_map.insert(pid, Some(handler));

        // Notify the executor
        //self.notify_new_process();

        Ok(pid)
    }

    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>
    {
        println!("* Kill process {:?} *", pid);
        
        match self.handler_map.get_mut(&pid) {
            Some(handler) => {
                match handler.take() {
                    Some(thread) => {

                        match self.thread_map.get_mut(&pid) {
                            Some(exec_thread) => {
                                println!("shutting down thread...");
                                exec_thread.shutdown_thread(pid);
                            }
                            None => {
                                // Error
                            }
                        }
                        
                        println!("joining...");
                        thread.join().unwrap();
                        println!("finished!");
                    },
                    None => {
                        // Error
                    }
                }
            },
            None => {
                // Error
            }
        }
        Ok(())
    }
}



/*
 *
 */
#[derive(Clone, Debug)]
struct ExecThread {
    boot_barrier: Arc<Barrier>,
    stop: Arc<Event>,
    stopped: Arc<Event>,
}

impl ExecThread {

    pub fn new(boot_barrier: Arc<Barrier>) -> Self
    {
        ExecThread { 
            boot_barrier,
            stop: Arc::new(Event::new()),
            stopped: Arc::new(Event::new()),
        }
    }

    fn boot_thread(
        &self, 
        tracee: Child,
        //address_ipv4: &str,
        //tracer_port: u16,
        //executor_port: u16,
    )
    {
        println!("[EXECUTOR] Start listening on {}:{}", IP_ADDRESS, EXECUTOR_PORT);
        let copy_stop = self.stop.clone();
        let copy_stopped = self.stopped.clone();
        let executor = ExecutorEngine::new(TargetArch::X86_64,
                                                           IP_ADDRESS,
                                                           EXECUTOR_PORT,
                                                           TRACER_PORT,
                                                           copy_stop,
                                                           copy_stopped
                                                          );
        //let exec_pid = tracee.id();

        let mem = read_process_memory_maps(tracee.id());
        print_memory_regions(&mem);

        self.run_thread(executor);
    }

    fn run_thread(&self, mut executor: ExecutorEngine)
    {
        executor.run();
    }

    pub fn shutdown_thread(&self, pid: Pid)
    {
        println!("[EXECUTOR] Thread executing process {} shutdown", pid);

        println!("killing...");
        ptrace::kill(pid).unwrap();
        println!("killed");

        if ! self.stopped.is_set() {
            self.stop.set();
            self.stopped.wait();
        }
        println!("thread stopped")
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

    let mut dbg = ExecDebugger::new();

    // Listen for incomming connection and order from a trace dgb
    println!("[EXECUTOR] Start debugger...");
    dbg.run();

    println!("[EXECUTOR] Stop debugger.");
}