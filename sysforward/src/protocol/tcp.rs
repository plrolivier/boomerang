/*
 * Not tested...
 */
use std::{
    net::{ SocketAddr, TcpStream },
    io::{ Error },
};

use crate::{
    protocol::{ Peer },
};



struct TcpPeer {
    stream: TcpStream,
}

impl TcpPerr {

    fn new(remote_addr: SocketAddr) -> Result<Self> {
        let stream = TcpStream::connect(remote_addr)?;
        Ok(TCPPeer { stream })
    }
}

impl Peer for TcpPeer {

    fn send(&self, data: &[u8]) -> Result<(), Error> {
        self.stream.write_all(data)?;
        Ok(())
    }

    fn receive(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.stream.read_to_end(buffer)?;
        Ok(buffer.len())
    }
}