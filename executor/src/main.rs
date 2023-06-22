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
    unistd::{ Pid },
};

use sysforward::{
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    protocol::control::{ Configuration, ControlChannel },
    executor_engine::{ ExecutorEngine, ExecutorCallback },
};


/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
static CONTROL_PORT: u16 = 31000;
static TRACER_PORT: u16 = 31001;
static EXECUTOR_PORT: u16 = 31002;



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

    fn spawn_process(&mut self, program: &str, prog_args: &[String]) -> Result<Pid, io::Error>
    {
        Ok(Pid::from_raw(10))
    }

    fn kill_process(&self, pid: Pid) -> Result<(), io::Error>
    {
        Ok(())
    }
}



/*
 *
 */
#[derive(Clone, Debug)]
struct ExecThread {
    boot_barrier: Arc<Barrier>,
}

impl ExecThread {

    fn boot_thread(
        &self, 
        tracee: Child,
        address_ipv4: &str,
        tracer_port: u16,
        executor_port: u16,
    )
    {
        println!("[EXECUTOR] Start listening on {}:{}", IP_ADDRESS, EXECUTOR_PORT);
        let mut executor = ExecutorEngine::new(TargetArch::X86_64, address_ipv4, executor_port, tracer_port);

        let mem = read_process_memory_maps(tracee.id());
        print_memory_regions(&mem);

        self.run_thread(executor);
    }

    fn run_thread(&self, mut executor: ExecutorEngine)
    {
        executor.run();
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