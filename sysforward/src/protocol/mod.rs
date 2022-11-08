/*
 *
 */
use std::{
    io::{prelude::*, BufReader, BufWriter, Result},
    net::{TcpListener, TcpStream},
};
use nix::unistd::Pid;
use crate::{
    { Syscall },

};

#[derive(Debug)]
pub enum Command {
    Ack,    // ack a command or signal an error with a command

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


pub struct Header {
    command: Command,
    pid: Pid,
    size: usize,
}

impl Header {
    pub fn new(command: Command, pid: Pid) -> Self {
        Self {
            command: command,
            pid: pid,
            size: 0,
        }
    }

    pub fn set_size(&mut self, len: usize) {
        self.size = len
    }

    pub fn to_json(&self) -> String {
        format!("{{\"command\": {:?}, \"pid\": {}, \"size\": {}}}\n", self.command, self.pid, self.size)
    }
}

pub struct Packet {
    header: Header,
    payload: SendSyscallEntryPayload, // TODO: place generic here
}

impl Packet {
    pub fn new(header: Header, payload: SendSyscallEntryPayload) -> Self {
        Self {
            header: header,
            payload: payload,
        }
    }

    pub fn to_json(&mut self) -> String {
        let payload = self.payload.to_json();
        self.header.set_size(payload.len());
        format!("{}{}", self.header.to_json(), payload)
    }
}

pub struct SendSyscallEntryPayload {
    syscall: String
}

impl SendSyscallEntryPayload {
    pub fn new(syscall: String) -> Self {
        Self {
            syscall: syscall,
        }
    }

    pub fn to_json(&self) -> String {
        format!("{{\"syscall\": {}}}\n", self.syscall)
    }
}




/*
 * In the long run, each PID should be assigned a stream and therefore an executor thread.
 */
pub struct Server { }

impl Server {
    
    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:33000").unwrap();

        /*
        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
        *
        * Listen to exactly 1 connection:
        */
        let (stream, addr) = listener.accept().unwrap();
        //println!("[server] Connection established with {addr:?}");
        self.handle_connection(stream);
    }


    fn handle_connection(&self, mut stream: TcpStream) {

        println!("[server] Connection established");

        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let mut reader = BufReader::new(stream.try_clone().unwrap()); // XXX: there is not way to
                                                                      // flush the reader....
        let mut buf = String::new();

        loop {
            /* Receive request */
            match reader.read_line(&mut buf) {
                Ok(0) => break,
                Ok(x) => (),
                Err(_) => break,
            }
            println!("[server] Received header:\n{}", buf);
            reader.read_line(&mut buf);
            println!("[server] Received payload:\n{}", buf);

            /* Parse request */
            // ...

            /* Proceed request */
            let response = format!("{{\"command\": {:?}, \"pid\": {}, \"size\": {}}}\n", Command::Ack, 0, 0);

            /* Send response */
            writer.write(response.as_bytes());
            writer.flush().unwrap();
        }
        println!("[server] Connection closed");
    }
}

/*
 * For now, just a wrapper around TCP stream, but in the long run must be ablo to handle different
 * type of connections (TCP, UDP, Serial, etc.).
 */
pub struct Client { 
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {

    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:33000").unwrap();
        Self {
            reader: BufReader::new(stream.try_clone().unwrap()),
            writer: BufWriter::new(stream),
        }
    }

    pub fn send(&mut self, buf: String) -> Result<usize> {
        let ret = self.writer.write(buf.as_bytes());
        self.writer.flush().unwrap();
        ret
    }

    pub fn receive(&mut self, buf: &mut String) -> Result<usize> {
        self.reader.read_line(buf)
    }
}

