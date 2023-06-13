/*
 *
 */
use std::{
    collections::{ HashMap },
    os::unix::process::{ CommandExt },
    process::{ exit, Child, Command },
    thread::{ Builder, JoinHandle },
    sync::{ Arc, Barrier },
    io::{self, prelude::*, BufReader, BufWriter },
    net::{TcpListener, TcpStream},
};

use nix::{
    sys::{
        ptrace,
        wait::{wait, waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::{ Pid },
};

use sysforward::{
    arch::TargetArch,
    memory::{ read_process_memory_maps, print_memory_regions },
    executor_engine::Executor,
    protocol::{ self, control },
};

use crate::{ IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT, tracer };



struct IoBuffer {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

struct ExecDebugger {
    handler_map: HashMap<Pid, Option<JoinHandle<()>>>,
    thread_map: HashMap<Pid, ExecThread>,

    listener: TcpListener,
}

impl ExecDebugger {

    pub fn new() -> Self
    {
        let listener = TcpListener::bind("127.0.0.1:31000").unwrap();
        Self {
            handler_map: HashMap::new(),
            thread_map: HashMap::new(),
            listener,
        }
    }

    pub fn listen(&mut self)
    {
        println!("[EXECUTOR] Listen for connections...");

        for mut stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);                                     // CONTINUE HERE    !!!!!!!
                }
                Err(e) => {
                    eprintln!("Fail to establish connection: {}", e);
                }
            }
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream)
    {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        //let mut reader = BufReader::new(stream);
        let mut writer = BufWriter::new(stream);
        //let mut io_buf = IoBuffer{ reader, writer };

        /* The main loop of the thread */
        loop {
            let message = self.receive_message(&mut reader);

            self.dispatch_message(message, &mut writer);
        }
    }

    fn receive_message(&self, reader: &mut BufReader<TcpStream>) -> protocol::control::Message
    {
        let mut buffer = String::new();

        reader.read_line(&mut buffer).unwrap();     // BUG ???
        println!("[EXECUTOR] Receive message: {}", buffer);

        // Should we use serde_json::from_reader instead ?
        let message: protocol::control::Message = serde_json::from_str(&buffer).unwrap();
        message 
    }

    fn dispatch_message(&mut self, message: protocol::control::Message, writer: &mut BufWriter<TcpStream>)
    {
        let mut data = message.payload.clone();
        match message.command {
            protocol::control::Command::Ack => {
                // Nothing to do
            },
            protocol::control::Command::NewProcess => {
                let payload: protocol::control::NewProcessRequestPayload = serde_json::from_str(&data).unwrap();
                let ret = self.new_process(payload);
                self.reply_new_process(ret, writer);
            },
            // TODO: implement the other commands...
        }
    }

    fn new_process(&mut self, payload: protocol::control::NewProcessRequestPayload) -> Result<(), io::Error>
    {
        // Spawn the child
        let child = unsafe {
            let mut command = Command::new("/bin/true");    // TODO: change /bin/true with a blob written to /tmp
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                Ok(())
            });

            command.spawn().expect("Failed to spawn child process")
        };


        let pid = Pid::from_raw(child.id() as i32);

        // Wait
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) => { /* ??? */ },
            _ => panic!("WaitStatus not handled"),
        };

        // Create the executing thread
        let boot_barrier = Arc::new(Barrier::new(2));

        let exec_thread = ExecThread { boot_barrier };
        let copy_exec_thread = exec_thread.clone();
        self.thread_map.insert(pid, exec_thread);

        let builder = Builder::new().name(child.id().to_string());  // TODO: sync PID with tracer...
        let handler = builder.spawn(move ||
            copy_exec_thread.boot_thread(
                child,
                payload.address_ipv4,
                payload.tracer_port,
                payload.executor_port
            )
        ).unwrap();
        self.handler_map.insert(pid, Some(handler));

        Ok(())
    }

    fn reply_new_process(&self, result: Result<(), io::Error>, writer: &mut BufWriter<TcpStream>)
    {
        let mut payload = String::new();
        match result {
            Ok(()) => {
                payload = serde_json::to_string(
                    &control::AckPayload {
                        command: protocol::control::Command::NewProcess,
                        result: None
                    }
                ).unwrap();
            },
            Err(e) => {
                payload = serde_json::to_string(
                    &control::AckPayload {
                        command: protocol::control::Command::NewProcess,
                        result: Some(e.to_string())
                    }
                ).unwrap();
            }
        }

        let message = control::Message {
            command: control::Command::Ack,
            payload: payload,
        };

        let mut data: String = serde_json::to_string(&message).unwrap();
        data.push_str("\n");
        println!("[EXECUTOR] Reply ack message: {}", data);

        let _ret = writer.write(data.as_bytes()).unwrap();
        writer.flush();

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
        let mut executor = Executor::new(TargetArch::X86_64, address_ipv4, executor_port, tracer_port);

        let mem = read_process_memory_maps(tracee.id());
        print_memory_regions(&mem);

        self.run_thread(executor);
    }

    fn run_thread(&self, mut executor: Executor)
    {
        executor.run();
    }

}



pub fn start_executor()
{

    let mut dbg = ExecDebugger::new();

    // Listen for incomming connection and order from a trace dgb
    dbg.listen();
}
