/*
 * For now, just a wrapper around TCP stream, but in the long run must be ablo to handle different
 * type of connections (TCP, UDP, Serial, etc.).
 */
use std::{
    io::{prelude::*, BufReader, BufWriter, Result},
    net::{ TcpStream },
};



pub struct Client { 
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {

    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:31000").unwrap();
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