/*
 *
 */
use std::{
    //collections::{ HashMap },
    //thread::{ Builder, JoinHandle },
    //os::unix::process::{ CommandExt },
    //process::{ exit, Child, Command },
    //sync::{ Arc, Barrier },
    io::{ self, BufRead, BufReader, BufWriter, Write },
    net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr}, fmt::format,
};

use crate::{
    tracer_engine::{ TracerCallback },
    executor_engine::{ ExecutorCallback },
};



/*
 * The control thread listen for commands from avatar.
 * It can be configured either as a tracer or executor and changed during runtime (TODO).
 */
pub enum Configuration {
    Tracer = 0,
    Executor = 1,
}


pub struct ControlChannel {
    configuration: Configuration,
    tracer: Option<Box<dyn TracerCallback>>,        // USe callbakc closure or trait ???
    executor: Option<Box<dyn ExecutorCallback>>,
    //stream: Option<TcpStream>,
    reader: Option<BufReader<TcpStream>>,
    writer: Option<BufWriter<TcpStream>>,
}

impl ControlChannel {

    pub fn new(configuration: Configuration, tracer: Option<Box<dyn TracerCallback>>, executor: Option<Box<dyn ExecutorCallback>>) -> Self
    {
        Self {
            configuration: configuration,
            tracer: tracer,
            executor: executor,
            //stream: None,
            reader: None,
            writer: None,
        }
    }

    /* 
    pub fn connect(&mut self, ip: Ipv4Addr, port: u16) -> Result<(), String>
    {
        let address = SocketAddr::new(ip.into(), port);
        match TcpStream::connect(address) {
            Ok(stream) => {
                //self.stream = Some(stream);
                self.reader = Some(BufReader::new(stream.try_clone().unwrap()));
                self.writer = Some(BufWriter::new(stream));
                println!("Connected with avatar2 on {:?}", address);
                Ok(())
            }
            Err(err) => {
                let msg = format!("Couldn't connect to avatar2: {}", err);
                eprintln!("{}", msg);
                Err(msg)
            }
        }
    }

    pub fn listen(&mut self)
    {
        /* Main loop listening for commands from avatar2 */
        loop {
            let message = self.receive_message();

            self.dispatch_message(message);
        }
    }
    */

    pub fn listen(&mut self, ip: Ipv4Addr, port: u16)
    {
        println!("Listen for connections...");
        let address = (ip, port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Fail to establish connection: {}", e);
                }
            }
        }
        println!("Finish listening")
    }

    fn handle_connection(&mut self, stream: TcpStream)
    {
        self.reader = Some(BufReader::new(stream.try_clone().unwrap()));
        self.writer = Some(BufWriter::new(stream));


        /* The main loop of the listening thread */
        loop {
            let mut buffer = String::new();

            match self.receive_message(&mut buffer) {
                Ok(_) => {
                    if buffer.trim().is_empty() {
                        println!("Connection closed");
                        break;
                    } else {
                        self.dispatch_message(buffer);
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }
    }

    fn receive_message(&mut self, buffer: &mut String) -> io::Result<usize>
    {

        let result = self.reader.as_mut().unwrap().read_line(buffer);
        println!("Message received: {:?}", buffer);
        result
    }

    fn dispatch_message(&mut self, message: String)
    {
        let command: Vec<&str> = message.split_whitespace().collect();

        match self.dispatch_command(command) {
            Ok(_) => {
                // TODO: Send Ack to avatar2
            },
            Err(msg) => {
                eprintln!("{}", msg);
                // TODO: Send back error message to avatar2
            }
        }
    }

    fn dispatch_command(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        match command[0] {
            "switch" => self.switch_configuration(command),

            "read_mem" => self.read_memory(command),
            "write_mem" => self.write_memory(command),

            "read_regs" => self.read_registers(command),
            "write_regs" => self.write_registers(command),

            "set_breakpoint" => self.set_breakpoint(command),
            "remove_breakpoint" => self.remove_breakpoint(command),

            "get_procmaps" => Err(format!("Not implemented")),
            //"" => Err(format!("Not implemented")),
            
            // Check for configuration specific commands
            _ => {
                match self.configuration {
                    Configuration::Tracer => self.dispatch_tracer(command),
                    Configuration::Executor => self.dispatch_executor(command),
                }
            }
        }

    }

    fn dispatch_tracer(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        match command[0] {
            "spawn_process" => self.tracer_spawn_process(command),
            "kill_process" => self.tracer_kill_process(command),
            "start_tracing" => self.tracer_start_tracing(command),
            "stop_tracing" => self.tracer_stop_tracing(command),
            //"" => Err(format!("Not implemented")),
            _ => {
                let msg = format!("[TRACER] Command not implemented: {}", command[0]);
                Err(msg)
            }
        }
    }

    fn dispatch_executor(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        match command[0] {
            "spawn_process" => self.executor_spawn_process(command),
            "kill_process" => self.executor_kill_process(command),
            //"" => Err(format!("Not implemented")),
            _ => {
                let msg = format!("[EXECUTOR] Command not implemented: {}", command[0]);
                Err(msg)
            }
        }
    }


    /* Function in common */

    fn switch_configuration(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let msg = format!("Switching between Tracer and Executor not supported yet :(");
        Err(msg)
    }

    fn read_registers(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command read register not implemented yet");
        Err(msg)
    }

    fn write_registers(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command write register not implemented yet");
        Err(msg)
    }

    fn read_memory(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command read memory not implemented yet");
        Err(msg)
    }

    fn write_memory(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command write memory not implemented yet");
        Err(msg)
    }

    fn set_breakpoint(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command set breakpoint not implemented yet");
        Err(msg)
    }

    fn remove_breakpoint(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command remove breakpoint not implemented yet");
        Err(msg)
    }
    

    /* Tracer related functions */
    
    fn tracer_spawn_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        println!("Warning mockup function: command {} not implemented yet", command[0]);

        let _program = command[1];
        //let args = command[2..];

        // TODO
        let pid: i32 = 10;

        // Reply
        let buffer = pid.to_be_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }

    fn tracer_kill_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        println!("Warning mockup function: command {} not implemented yet", command[0]);

        let _pid = command[1];

        // TODO
        let mut ack = String::new();
        ack.push_str("ACK");

        // Reply
        let buffer = ack.as_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }

    fn tracer_start_tracing(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        println!("Warning mockup function: command {} not implemented yet", command[0]);
        let _pid = command[1];

        // TODO
        let mut ack = String::new();
        ack.push_str("ACK");

        let buffer = ack.as_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }
    
    fn tracer_stop_tracing(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        println!("Warning mockup function: command {} not implemented yet", command[0]);
        let _pid = command[1];

        // TODO
        let mut ack = String::new();
        ack.push_str("ACK");

        let buffer = ack.as_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }


    /* Executor related functions */

    fn executor_spawn_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        println!("Warning mockup function: command {} not implemented yet", command[0]);

        let _program = command[1];
        //let args = command[2..];

        // TODO
        let pid: i32 = 20;

        // Reply
        let buffer = pid.to_be_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }

    fn executor_kill_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        println!("Warning mockup function: command {} not implemented yet", command[0]);

        let _pid = command[1];

        // TODO
        let mut ack = String::new();
        ack.push_str("ACK");

        // Reply
        let buffer = ack.as_bytes();
        self.writer.as_mut().unwrap().write(&buffer).unwrap();
        self.writer.as_mut().unwrap().flush().unwrap();

        Ok(())
    }


}



/* 
pub struct ControlChannelServer {
    reader: Option<BufReader<TcpStream>>,
    writer: Option<BufWriter<TcpStream>>,
}

impl ControlChannelServer {

    pub fn new(configuration: Configuration) -> Self
    {
        Self {
            configuration,
            reader: None,
            writer: None,
        }

    }

}
*/

