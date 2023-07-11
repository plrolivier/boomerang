/*
 *
 */
use std::{
    str::FromStr,
    //collections::{ HashMap },
    //thread::{ Builder, JoinHandle },
    //os::unix::process::{ CommandExt },
    //process::{ exit, Child, Command },
    //sync::{ Arc, Barrier },
    io::{ self, BufRead, BufReader, BufWriter, Write },
    net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr}, fmt::format,
};

use nix::{
    unistd::{ Pid },
    sys::signal::Signal,
};

use crate::{
    tracer::{ TracerCallback },
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
                // TODO: Send Ack to avatar2 ?
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
        match command[0] {
            "spawn_process" => self.tracer_spawn_process(command),
            "kill_process" => self.tracer_kill_process(command),
            "start_tracing" => self.tracer_start_tracing(command),
            "cont_tracing" => self.tracer_cont_tracing(command),
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
        let program = command[1].to_string();
        let args_str: Vec<&str> = command[2..].to_vec();
        let args: Vec<String> = args_str.iter().map(|&s| s.to_string()).collect();

        match self.tracer.as_mut() {
            Some(tracer) => {
                // TODO: match on the value return by spawn_process
                let pid = tracer.spawn_process(program, args).unwrap();

                let buffer = pid.as_raw().to_be_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
        Ok(())
    }

    fn tracer_kill_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let pid = command[1];
        let pid = FromStr::from_str(pid).unwrap();
        let pid = Pid::from_raw(pid);

        match self.tracer.as_mut() {
            Some(tracer) => {
                // TODO: match on the value return by spawn_process
                tracer.kill_process(pid).unwrap();

                let mut ack = String::new();
                ack.push_str("ACK");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
        Ok(())
    }

    fn tracer_start_tracing(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let pid = command[1];
        let pid = FromStr::from_str(pid).unwrap();
        let pid = Pid::from_raw(pid);

        match self.tracer.as_mut() {
            Some(tracer) => {
                // TODO: match on the value return by spawn_process
                tracer.start_tracing(pid).unwrap();

                let mut ack = String::new();
                ack.push_str("ACK");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
        Ok(())
    }

    fn tracer_cont_tracing(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let pid = command[1];
        let pid = FromStr::from_str(pid).unwrap();
        let pid = Pid::from_raw(pid);

        let sig = command[2];
        let sig = FromStr::from_str(sig).unwrap();
        let signal: Option<Signal>;
        // Linux signals
        match sig {
            1 => signal = Some(Signal::SIGHUP),
            2 => signal = Some(Signal::SIGINT),
            3 => signal = Some(Signal::SIGQUIT),
            4 => signal = Some(Signal::SIGKILL),
            5 => signal = Some(Signal::SIGTRAP),
            6 => signal = Some(Signal::SIGABRT),
            7 => signal = Some(Signal::SIGBUS),
            8 => signal = Some(Signal::SIGFPE),
            9 => signal = Some(Signal::SIGKILL),
            10 => signal = Some(Signal::SIGUSR1),
            11 => signal = Some(Signal::SIGSEGV),
            12 => signal = Some(Signal::SIGUSR2),
            13 => signal = Some(Signal::SIGPIPE),
            14 => signal = Some(Signal::SIGALRM),
            15 => signal = Some(Signal::SIGTERM),
            //16
            17 => signal = Some(Signal::SIGCHLD),
            18 => signal = Some(Signal::SIGCONT),
            19 => signal = Some(Signal::SIGSTOP),
            20 => signal = Some(Signal::SIGTSTP),
            21 => signal = Some(Signal::SIGTTIN),
            22 => signal = Some(Signal::SIGTTOU),
            23 => signal = Some(Signal::SIGURG),
            24 => signal = Some(Signal::SIGXCPU),
            25 => signal = Some(Signal::SIGXFSZ),
            26 => signal = Some(Signal::SIGVTALRM),
            27 => signal = Some(Signal::SIGPROF),
            28 => signal = Some(Signal::SIGWINCH),
            29 => signal = Some(Signal::SIGIO),
            30 => signal = Some(Signal::SIGPWR),
            31 => signal = Some(Signal::SIGSYS),
            _ => signal = None,
        }

        match self.tracer.as_mut() {
            Some(tracer) => {
                tracer.cont_tracing(pid, signal).unwrap();

                let mut ack = String::new();
                ack.push_str("ACK");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        Ok(())
    }
    
    fn tracer_stop_tracing(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let pid = command[1];
        let pid = FromStr::from_str(pid).unwrap();
        let pid = Pid::from_raw(pid);

        match self.tracer.as_mut() {
            Some(tracer) => {
                // TODO: match on the value return by spawn_process
                tracer.stop_tracing(pid).unwrap();

                let mut ack = String::new();
                ack.push_str("ACK");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
        Ok(())
    }


    /* Executor related functions */

    fn executor_spawn_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let program = command[1];
        let args = &command[2..];

        match self.executor.as_mut() {
            Some(executor) => {
                // TODO: match on the value return by spawn_process
                let pid = executor.spawn_process(program, args).unwrap();

                let buffer = pid.as_raw().to_be_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
        Ok(())
    }

    fn executor_kill_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        let pid = command[1];
        let pid = FromStr::from_str(pid).unwrap();
        let pid = Pid::from_raw(pid);

        match self.executor.as_mut() {
            Some(executor) => {
                // TODO: match on the value return by spawn_process
                executor.kill_process(pid).unwrap();

                let mut ack = String::new();
                ack.push_str("ACK");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            },
            None => {
                let mut ack = String::new();
                ack.push_str("ERR");
                let buffer = ack.as_bytes();
                self.writer.as_mut().unwrap().write(&buffer).unwrap();
            }
        }

        self.writer.as_mut().unwrap().flush().unwrap();

        // when should we return an error ?
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

