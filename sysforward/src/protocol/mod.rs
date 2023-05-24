/*
 *
 */
//pub mod client;
//pub mod server;

//pub mod udpclient;
//pub mod udpserver;

pub mod udp;
//pub mod tcp;

//use std::{
//    io::{prelude::*, BufReader, BufWriter, Result},
//    net::{TcpListener, TcpStream},
//    os::unix::io::AsRawFd,
//};
//use nix::sys::socket::{self, sockopt::ReusePort};

//use serde::{Serialize, Deserialize};

//use crate::{
//    syscall::{ Syscall },
//
//};


// /* Tracer */
// fn send_syscall_entry(self, Syscall) { }
// 
// fn wait_syscall_entry(self) { }
// 
// fn send_syscall_exit(self, Syscall) { }
// 
// fn wait_syscall_exit(self) { }
// 
// 
// /* Executor */
// fn receive_syscall_entry(self) -> Syscall { }
