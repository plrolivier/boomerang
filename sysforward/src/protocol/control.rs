/*
 *
 */
use std::{
    //collections::{ HashMap },
    //thread::{ Builder, JoinHandle },
    //os::unix::process::{ CommandExt },
    //process::{ exit, Child, Command },
    //sync::{ Arc, Barrier },
    io::{ BufRead, BufReader, BufWriter },
    net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr}, fmt::format,
};




/*
 * The control thread listen for commands from avatar.
 * It can be configured either as a tracer or executor and changed during runtime (TODO).
 */
pub enum Configuration {
    Tracer,
    Executor,
}


pub struct ControlChannel {
    configuration: Configuration,
    //stream: Option<TcpStream>,
    reader: Option<BufReader<TcpStream>>,
    writer: Option<BufWriter<TcpStream>>,
}

impl ControlChannel {

    pub fn new(configuration: Configuration) -> Self
    {
        Self {
            configuration,
            //stream: None,
            reader: None,
            writer: None,
        }
    }

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

    fn receive_message(&mut self) -> String
    {
        let mut buffer = String::new();

        self.reader.as_mut().unwrap().read_line(&mut buffer).unwrap();
        println!("Receive message: {}", buffer);
        buffer 
    }

    fn dispatch_message(&mut self, message: String)
    {
        let command: Vec<&str> = message.trim().split_whitespace().collect();

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
            "spawn_process" => Err(format!("Not implemented")),
            "kill_process" => Err(format!("Not implemented")),
            "start_tracing" => Err(format!("Not implemented")),
            "stop_tracing" => Err(format!("Not implemented")),
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
            "spawn_process" => Err(format!("Not implemented")),
            "kill_process" => Err(format!("Not implemented")),
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
    
    fn tracer_start_process(&mut self, command: Vec<&str>) -> Result<(), String>
    {
        // TODO
        let msg = format!("Command start process not implemented yet");
        Err(msg)
    }
    


    /* Executor related functions */


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
    }

    fn handle_connection(&mut self, stream: TcpStream)
    {
        self.reader = Some(BufReader::new(stream.try_clone().unwrap()));
        self.writer = Some(BufWriter::new(stream));

        /* The main loop of the listening thread */
        loop {
            let message = self.receive_message();

            self.dispatch_message(message);
        }
    }

}
*/

