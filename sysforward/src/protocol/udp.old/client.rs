/*
 *
 */
use std::net::{UdpSocket, SocketAddr};


pub struct UdpClient {
    server_addr : SocketAddr,
    socket: UdpSocket,
}


impl UdpClient {

    pub fn new() -> Result<Self, std::io::Error> {
        let server_addr: SocketAddr = "127.0.0.1:31000".parse().unwrap();
        let socket = UdpSocket::bind("127.0.0.1:32000")?;
        Ok(UdpClient { server_addr, socket })
    }

    pub fn send(&self, message: &str) -> Result<(), std::io::Error> {
        self.socket.send_to(message.as_bytes(), self.server_addr)?;
        Ok(())
    }

    pub fn receive(&self) -> Result<(Vec<u8>, SocketAddr), std::io::Error> {
        let mut buf = [0; 1024];
        let (len, src) = self.socket.recv_from(&mut buf)?;
        let data = buf[..len].to_vec();
        Ok((data, src))
    }
}
