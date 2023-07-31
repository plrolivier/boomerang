/*
 *
 */
use std::{
    io::{prelude::*, BufReader, BufWriter, Result},
    net::{TcpListener, TcpStream},
    os::unix::io::AsRawFd,
};
use nix::sys::socket::{self, sockopt::ReusePort};
use serde::{Serialize, Deserialize};
use crate::{
    syscall::{ Syscall },

};

#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum Command {
    Ack = 0,    // ack a command or signal an error with a command

    NotifyNewProcess    = 1,
    SendSyscallEntry    = 2,
    NotifySyscallExit   = 3,
    ReturnSyscallExit   = 4,
    ReturnDecision      = 5,
    NotifySignal        = 6,

    ReadArguments       = 7,
    WriteArgument       = 8,
    WriteArguments      = 9,

    ReadRegisters       = 10,
    WriteRegister       = 11,
    WriteRegisters      = 12,

    ReadMemory          = 13,
    ReadString          = 14,
    WriteMemory         = 15,
}


#[derive(Serialize)]
pub struct Packet {
    header: Header,
    payload: SendSyscallEntryPayload, // TODO: place generic here
}

impl Packet {
    pub fn new(header: Header, payload: SendSyscallEntryPayload) -> Self {
        Self {
            header: header,
            payload: payload 
        }
    }

    pub fn to_json(&mut self) -> String {
        format!("{{\"header\":{},\"payload\":{}}}", self.header.to_json(), self.payload.to_json())
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pid: i32,
    //#[serde(rename="cmd")]
    command: Command,
    size: usize,
}

impl Header {
    pub fn new(pid: i32, command: Command, size: usize) -> Self {
        Self {
            pid: pid,
            command: command,
            size: size,
        }
    }

    pub fn to_json(&self) -> String {
        // the header is always 49 bytes
        format!("{{\"pid\":{:07},\"command\":{:0>2},\"size\":{:#08x}}}", self.command as u8, self.pid, self.size)
    }
}


#[derive(Serialize)]
pub struct SendSyscallEntryPayload {
    syscall: Syscall
}

impl SendSyscallEntryPayload {
    //pub fn new(syscall: String) -> Self {
    pub fn new(syscall: &Syscall) -> Self {
        Self {
            syscall: syscall.clone(),
        }
    }

    pub fn to_json(&self) -> String {
        format!("{{\"syscall\": {}}}\n", self.syscall.to_json())
    }
}




/*
 * In the long run, each PID should be assigned a stream and therefore an executor thread.
 */
pub struct Server { }

impl Server {
    
    pub fn listen(&self) {
    //pub fn listen(&self) -> TcpStream {
        let listener = TcpListener::bind("127.0.0.1:33000").unwrap();
        socket::setsockopt(listener.as_raw_fd(), ReusePort, &true).unwrap();

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
        //stream
    }


    fn handle_connection(&self, mut stream: TcpStream) {

        println!("[server] Connection established");

        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let mut reader = BufReader::new(stream.try_clone().unwrap()); // XXX: there is no way to
                                                                      // flush the reader....

        loop {
            /* Receive request */
            let mut header = String::new();
            match reader.read_line(&mut header) {
                Ok(0) => break,
                Ok(_) => (),
                Err(_) => break,
            }
            println!("[server] Received header: {}", header);
            let mut payload = String::new();
            reader.read_line(&mut payload);
            println!("[server] Received payload: {}", payload);

            /* Parse request */
            let request_header: Header = serde_json::from_str(&header).unwrap();
            // TODO: deserialize payload

            /* Proceed request */

            /* Prepare response */
            let response_header = Header::new(request_header.pid, Command::Ack, 0);
            let str_response_header = serde_json::to_string(&response_header).unwrap();
            let response = format!("{}\n", str_response_header);

            /* Send response */
            writer.write(response.as_bytes());
            writer.flush().unwrap();
        }
        println!("[server] Connection closed");
    }

    /*
    fn receive_syscall(&self) -> Packet {
        let mut reader = BufReader::new(stream.try_clone().unwrap()); // XXX: there is no way to
                                                                      //
        /* Receive request */
        let mut header = String::new();
        match reader.read_line(&mut header) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        }
        println!("[server] Received header: {}", header);
        let mut payload = String::new();
        reader.read_line(&mut payload);
        println!("[server] Received payload: {}", payload);

        /* Parse request */
        let request_header: Header = serde_json::from_str(&header).unwrap();

        // TODO: deserialize payload
        // let request_payload: SendSyscallEntryPayload = serde_json::from_str(&payload).unwrap();
        let request_payload
        let request = {
            header: request_header,
            payload: request_payload.syscall,
        }
        request
    }
    */
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

