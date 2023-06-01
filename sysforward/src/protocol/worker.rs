/*
 * replace executor with invoker
 */
use std::{
    net::{ SocketAddr, Ipv4Addr },
    io::{ Error, ErrorKind },
};

use crate::{
    syscall::{ Syscall },
    protocol::{ 
        { IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT },
        Peer,
        udp::{ UdpPeer },
        message::{ Message, Header, Command, Payload },
    }, 
};



pub struct Worker { 
    connection: Box<dyn Peer>,

    /*  */
}

impl Worker {

    pub fn new() -> Self
    {
        // For now use hardcoded address and UDP
        let ip = IP_ADDRESS.parse::<Ipv4Addr>().expect("Invalid IPv4 address");

        let local_addr = SocketAddr::new(ip.into(), EXECUTOR_PORT);
        let remote_addr = SocketAddr::new(ip.into(), TRACER_PORT);

        let connection = UdpPeer::new(local_addr, remote_addr).expect("Unable to create UDP Peer");
        Worker {
            connection: Box::new(connection),
        }
    }


    /*  */

    pub fn return_syscall_exit(&self, syscall: &Syscall)
    {
        // Craft the message
        let payload_data = serde_json::to_vec(syscall).expect("Fail converting sycall to JSON");
        let payload = Payload::new(Some(payload_data));
        let header = Header::new(Command::ReturnSyscallExit, payload.len());
        let message = Message::new(header, payload);
        println!("[EXECUTOR] Send {:?}", message);

        // Send it
        let data = serde_json::to_vec(&message).expect("Fail converting message");
        self.connection.send(&data);
    }

    pub fn notify_signal(&self, signo: u32)
    {
        // TODO
        println!("TODO: notify_signal()");
    }


    /*  */

    /* 
    pub fn receive_message(&self) -> Result<Message, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        let len = self.connection.receive(&mut buffer).expect("Fail receiving peer data");
        println!("[EXECUTOR RECEIVE {} bytes] {:?}", len, buffer);

        let message = serde_json::from_slice(&buffer).expect("Fail to deserialize JSON");
        Ok(message)

        //self.parse_message(message);
    }
    */

    //pub fn receive_message(&self) -> Result<Message, Error>
    pub fn receive_message(&self) -> Message
    {
        let mut buffer = vec![0u8; 1024];
        let size: usize = self.connection.receive(&mut buffer).expect("blabl");
        println!{"[EXECUTOR] Receive {} bytes: {:?}", size, buffer};

        let data: &[u8] = &buffer[..size];
        println!{"[EXECUTOR] Receive Header {} bytes: {:?}", size, data};

        let header = serde_json::from_slice::<Header>(data).expect("not ok");
        println!("[EXECUTOR] Header: {:?}", header);
        let payload = self.receive_payload(header.size, data);
        Message { header, payload }
    }

    fn receive_payload(&self, size: usize, start_payload: &[u8]) -> Payload
    {
        let mut raw_payload: Vec<u8> = Vec::new();
        raw_payload.extend_from_slice(start_payload);
        raw_payload.resize(size, 0);

        let mut data: &mut [u8] = &mut raw_payload[start_payload.len()..];
        let len: usize = self.connection.receive(&mut data).expect("not okok");

        //let mut raw_payload: Vec<u8> = Vec::with_capacity(size);
        //let len: usize = self.connection.receive(&mut raw_payload).expect("not okok");

        println!{"[EXECUTOR] Receive Payload {} bytes: {:?}", len, data};

        if len != size {
            panic!("Read {} bytes for payload instead of {}", len + start_payload.len(), size);
        }

        let payload = serde_json::from_slice::<Payload>(&raw_payload).expect("msg");
        payload
    }


    /* 
     * The parser is located in the executor.
     * It is not ideal because it means the executor is dependent of how the protocol is working...
    fn parse_message(&self, message: Message)
    {
        let header = message.header;
        let payload = message.payload;

        match header.command {
            Command::SendSyscallEntry => {
                self.on_send_syscall_entry(payload.unwrap());
            },
            Command::NotifyNewProcess => {
                self.on_notify_new_process(payload.unwrap());
            },
            _ => {
                println!("[EXECUTOR] Command {:?} not implemented", header.command);
            }
        }
    }

    fn on_send_syscall_entry(&self, payload: Vec<u8>)
    {
        let syscall: Syscall = serde_json::from_slice(&payload).expect("Fail to deserialize syscall entry");
        //(self.invoke_syscall_entry)(self.invoker, syscall);
        // return syscall
    }

    fn on_notify_new_process(&self, payload: Vec<u8>)
    {
        //(self.invoke_new_process)(self.invoker);
        // idk
    }
     */

}
