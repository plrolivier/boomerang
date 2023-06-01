/*
 *
 */
use std::{
    net::{UdpSocket, SocketAddr},
    io::{ Error },
};

use crate::{
    protocol::{ Peer },
};



pub struct UdpPeer {
    local_socket: UdpSocket,
    remote_addr : SocketAddr,
}

impl UdpPeer {
    pub fn new(local_addr: SocketAddr, remote_addr: SocketAddr) -> Result<Self, Error> {
        let local_socket = UdpSocket::bind(local_addr)?;
        Ok(UdpPeer {
            local_socket,
            remote_addr,
        })
    }
}

impl Peer for UdpPeer {

    fn send(&self, data: &[u8]) -> Result<(), Error> {
        self.local_socket.send_to(data, self.remote_addr)?;
        Ok(())
    }

    /*
    fn receive(&self, buffer: &mut Vec<u8>) -> Result<usize, Error> {
        let mut buf = vec![0u8; buffer.capacity()];
        let (size, _) = self.local_socket.recv_from(&mut buf)?;
        buffer.clear();
        buffer.extend_from_slice(&buf[..size]);
        Ok(size)
    }
    */

    fn receive(&self, buffer: &mut [u8]) -> Result<usize, Error>
    {
        let (size, addr) = self.local_socket.recv_from(buffer)?;
        Ok(size)
        /*
        if self.remote_addr.eq(&addr) {
            Ok(size)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, "Socket address is different from remote address"))
        }
        */
    }


    //pub fn new() -> Result<Self, std::io::Error> {
        //let server_addr: SocketAddr = "127.0.0.1:31000".parse().unwrap();
        //let socket = UdpSocket::bind("127.0.0.1:32000")?;
        //Ok(UdpClient { server_addr, socket })
    //}

    //pub fn send(&self, message: &str) -> Result<(), std::io::Error> {
        //self.socket.send_to(message.as_bytes(), self.server_addr)?;
        //Ok(())
    //}

    //pub fn receive(&self) -> Result<(Vec<u8>, SocketAddr), std::io::Error> {
        //let mut buf = [0; 1024];
        //let (len, src) = self.socket.recv_from(&mut buf)?;
        //let data = buf[..len].to_vec();
        //Ok((data, src))
    //}
}
