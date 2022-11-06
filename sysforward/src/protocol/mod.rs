
use std::{
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub enum Command {
    Error,

    NotifyNewProcess,
    SendSyscallEntry,
    NotifySyscallExit,
    ReturnSyscallExit,
    ReturnDecision,
    NotifySignal,

    ReadArguments,
    WriteArgument,
    WriteArguments,

    ReadRegisters,
    WriteRegister,
    WriteRegisters,

    ReadMemory,
    ReadString,
    WriteMemory,
}

/*
 * In the long run, each PID should be assigned a stream and therefore an executor thread.
 */
pub struct Server { }

impl Server {
    
    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:31000").unwrap();

        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }


    fn handle_connection(&self, mut stream: TcpStream) {

        println!("[server] Connection established");
        let mut wstream = stream.try_clone().unwrap();
        let mut reader = BufReader::new(&mut stream);
        let mut buf = String::new();

        loop {
            /* Receive request */
            reader.read_to_string(&mut buf).unwrap();
            println!("[server] Received new request:\n{}", buf);

            /* Parse request */
            // ...

            /* Proceed request */
            let payload = format!("{{}}");

            /* Send response */
            let response = format!("{{\"command\": {:?}, \"pid\": {}, \"payload\": {}}}", Command::SendSyscallEntry, 0, payload);
            println!("[server] Send response:\n{}", response);
            wstream.write(response.as_bytes());
        }
        println!("[server] Connection closed");
    }
}

/*
 * For now, just a wrapper around TCP stream, but in the long run must be ablo to handle different
 * type of connections (TCP, UDP, Serial, etc.).
 */
pub struct Client { 
    stream: TcpStream,
}

impl Client {

    pub fn new() -> Self {
        Self {
            stream: TcpStream::connect("127.0.0.1:31000").unwrap(),
        }
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf)
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stream.read(buf)
    }
}
