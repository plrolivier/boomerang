/*
 * In the long run, each PID should be assigned a stream and therefore an executor thread.
 */
use std::{
    io::{prelude::*, BufReader },
    net::{TcpListener, TcpStream},
};



pub struct Server {
    listener: TcpListener,
    stream: Option<TcpStream>,
}

impl Server {

    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:31000").unwrap();
        Self {
            listener,
            stream: None,
        }
    }

    pub fn start_server(&mut self) {
        self.listen();
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            self.stream = Some(stream.unwrap());
            break;
        }
    }

    pub fn receive(&mut self) -> String {
        if let Some(stream) = &mut self.stream {
            let mut reader = BufReader::new(stream);
            let mut buffer = String::new();
            println!("reading line...");
            reader.read_line(&mut buffer).unwrap();             // BUG HERE NO message received
            println!("line read: {}", buffer);
            buffer
        } else {
            String::from("ERROR")
        }
    }

    pub fn reply(&mut self, message: &str) {
        if let Some(stream) = &mut self.stream {
            stream.write_all(message.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        // else => nothing...
    }

    /*
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