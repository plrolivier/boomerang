/*
 *
 */
use std::{
    net::{ SocketAddr, UdpSocket, Ipv4Addr },
};

use crate::{
    syscall::{ Syscall },
};



/* 
 * Header used for metadata, for instance sending the number of bytes of the payload.
 * Note: as long as only the size is really useful, no need for a struct.
 */
const HEADER_SIZE: usize = 8;
/*
#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Header {
    pub size: usize,
    //pid: i32,
}

impl Header {
    pub fn new(size: usize) -> Self
    {
        Self { size }
    }
}
 */


/*
 * A Peer represents the endpoint of a connection between a tracer and an executor.
 * It is used to send syscall information through UDP.
 */
pub struct Peer {
    local_socket: UdpSocket,
    //local_address: SocketAddr,
    remote_address: SocketAddr,
}

impl Peer {

    pub fn new(local_addr: SocketAddr, remote_address: SocketAddr) -> Self {
        let local_socket = UdpSocket::bind(local_addr).unwrap();
        Self { local_socket, remote_address }
    }

    pub fn send(&self, data: &[u8]) -> Result<(), std::io::Error> 
    {
        // Craft the header
        /* 
        let header = Header::new(data.len());
        //let header = serde_json::to_vec(&header).expect("Fail to serialize header to JSON");
        let header = serde::Serialize
        let message = [&header, data].concat();
        println!("Header: {:?}", header);
        */
        let header = data.len().to_be_bytes();
        let message: Vec<u8> = [&header[..HEADER_SIZE], data].concat();

        // Send the message
        let _size: usize = self.local_socket.send_to(&message, self.remote_address)?;
        //println!("Sent {} bytes", size);
        Ok(())
    }

    pub fn receive(&self) -> Result<(Vec<u8>, usize), std::io::Error>
    {
        /*
        const HEADER_SIZE: usize = size_of::<Header>();
        let mut header_buffer = [0u8; HEADER_SIZE];
        let (count, _addr): (usize, SocketAddr) = self.local_socket.peek_from(&mut header_buffer)?;
        if count != HEADER_SIZE {
            panic!("Fail to read header");
        }
        println!("HEADER_SIZE = {}", HEADER_SIZE);
        println!("Header: {:?}", header_buffer);
        let header: Header = serde_json::from_slice(&header_buffer[..HEADER_SIZE]).expect("Fail to deserialize header from JSON");
        */

        // Read header containing the size of the payload
        let mut header = [0u8; HEADER_SIZE];
        let (count, _addr): (usize, SocketAddr) = self.local_socket.peek_from(&mut header)?;
        if count != HEADER_SIZE { panic!("Fail to read header"); }
        let size = usize::from_be_bytes(header);

        // Read the payload
        let mut message: Vec<u8> = vec![0u8; HEADER_SIZE + size];
        let (size, _addr): (usize, SocketAddr) = self.local_socket.recv_from(&mut message)?;
        //println!("Received {} bytes", size);
        let payload = message.split_off(HEADER_SIZE);
        Ok((payload, size))
    }

}




/*
 * The dispatcher...
 */
pub struct Client { 
    connection: Peer,
}

impl Client {

    pub fn new(
        ipv4_addr: &str,
        tracer_port: u16,
        executor_port: u16,
    ) -> Self 
    {
        // For now use hardcoded address and UDP
        let ip = ipv4_addr.parse::<Ipv4Addr>().expect("Invalid IPv4 address");

        let local_addr = SocketAddr::new(ip.into(), tracer_port);
        let remote_addr = SocketAddr::new(ip.into(), executor_port);

        let connection = Peer::new(local_addr, remote_addr);
        Client { connection }
    }

    pub fn send_syscall_entry(&self, syscall: &Syscall)
    {
        // Craft the message
        let data: String = serde_json::to_string(syscall).expect("Fail to serialize syscall to JSON");
        //println!("[TRACER] Send syscall: {:?}", data);

        // Send the message
        self.connection.send(&data.as_bytes()).expect("Fail to send syscall entry");

        // Wait for the reply
        let (buffer, _len): (Vec<u8>, usize) = self.connection.receive().expect("Error receiving syscall reply message");
        //println!("[TRACER] Received {} bytes: {:?}", len, buffer);

        let _remote_syscall: Syscall = serde_json::from_slice(&buffer).expect("Fail to deserialize Syscall from JSON");
        //remote_syscall
    }

}


/*
 * The worker...
 */
pub struct Server { 
    connection: Peer,
}

impl Server {

    pub fn new(
        ipv4_addr: &str,
        executor_port: u16,
        tracer_port: u16,
    ) -> Self
    {
        // For now use hardcoded address and UDP
        let ip: Ipv4Addr = ipv4_addr.parse::<Ipv4Addr>().expect("Invalid IPv4 address");

        let local_addr = SocketAddr::new(ip.into(), executor_port);
        let remote_addr = SocketAddr::new(ip.into(), tracer_port);

        let connection = Peer::new(local_addr, remote_addr);
        Server { connection }
    }


    pub fn receive_syscall(&self) -> Syscall
    {
        // Read socket
        //let mut buffer: Vec<u8> = vec![0; 1024];
        let (buffer, _len): (Vec<u8>, usize)  = self.connection.receive().expect("Error receiving syscall message");
        //println!("[EXECUTOR] Received {} bytes: {:?}", len, buffer);

        // Parse syscall
        let syscall = serde_json::from_slice(&buffer).expect("Fail to deserialize Syscall from JSON");
        syscall
    }


    pub fn return_syscall_exit(&self, syscall: &Syscall)
    {
        // Craft the message
        let message: String = serde_json::to_string(syscall).expect("Fail to serialize syscall to JSON");
        let data: &[u8] = message.as_bytes();
        //println!("[EXECUTOR] Send syscall: {:?}", message);

        // Send the message
        self.connection.send(&data).expect("Failt to return syscall exit");
    }
}