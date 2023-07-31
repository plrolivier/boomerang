/*
 *
 */
use std::{
    net::{ SocketAddr, Ipv4Addr },
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



pub struct Dispatcher { 
    //connection: Box<dyn Peer>,
    connection: Peer,
}

impl Dispatcher {

    pub fn new() -> Self {
        // For now use hardcoded address and UDP
        let ip = IP_ADDRESS.parse::<Ipv4Addr>().expect("Invalid IPv4 address");

        let local_addr = SocketAddr::new(ip.into(), TRACER_PORT);
        let remote_addr = SocketAddr::new(ip.into(), EXECUTOR_PORT);

        let connection = UdpPeer::new(local_addr, remote_addr).expect("Unable to create UDP Peer");
        Dispatcher { 
            connection: Box::new(connection) 
        }
    }

    pub fn notify_new_process(&self) {
        // TODO
    }

    pub fn send_syscall_entry(&self, syscall: &Syscall) {

        // Craft the message
        let payload = Payload::new(Some(serde_json::to_vec(syscall).expect("Fail converting sycall to JSON")));
        let mut data_payload= serde_json::to_vec(&payload).expect("Failt converting header");

        let header = Header::new(Command::SendSyscallEntry, data_payload.len());
        let mut data_header = serde_json::to_vec(&header).expect("Failt converting header");

        println!("[TRACER] Send {:?}{:?}", header, payload);
        println!("[TRACER] Send Header {} bytes: {:?}", data_header.len(), data_header);
        println!("[TRACER] Send Payload {} bytes: {:?}", data_payload.len(), data_payload);

        //self.connection.send(&data_header);
        //self.connection.send(&data_payload);
        data_header.append(&mut data_payload);
        self.connection.send(&data_header);

        // Wait for reply
        let mut buffer: Vec<u8> = Vec::new();
        self.connection.receive(&mut buffer);
        print!("[TRACER RECEIVED] {:?}", buffer);
    }

}