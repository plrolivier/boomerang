/*
 *
 */
use std::net::{UdpSocket, SocketAddr};


pub struct UdpServer {
    client_addr : SocketAddr,
    socket: UdpSocket,
}

impl UdpServer {
    //pub fn new(addr: &str) -> Result<Self, std::io::Error> {
    pub fn new() -> Result<Self, std::io::Error> {
        let client_addr: SocketAddr = "127.0.0.1:32000".parse().unwrap();
        let addr: SocketAddr = "127.0.0.1:31000".parse().unwrap();
        let socket = UdpSocket::bind(addr)?;
        Ok(UdpServer { client_addr, socket })
    }

    pub fn receive(&self) -> Result<(Vec<u8>, SocketAddr), std::io::Error> {
        let mut buf = [0; 10*1024];
        let (len, src) = self.socket.recv_from(&mut buf)?;
        let data = buf[..len].to_vec();
        Ok((data, src))
    }


    pub fn reply(&self, message: &str) -> Result<(), std::io::Error> {
        self.socket.send_to(message.as_bytes(), self.client_addr)?;
        Ok(())
    }
}

