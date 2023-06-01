/*
 *
 */

use serde::{Serialize, Deserialize};



#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum Command {
    Ack = 0,    // ack a command or signal an error with a command

    NotifyNewProcess    = 1,
    SendSyscallEntry    = 2,
    NotifySyscallExit   = 3,
    ReturnSyscallExit   = 4,
    ReturnDecision      = 5,
    NotifySignal        = 6,

    //ReadArguments       = 7,
    //WriteArgument       = 8,
    //WriteArguments      = 9,

    //ReadRegisters       = 10,
    //WriteRegister       = 11,
    //WriteRegisters      = 12,

    //ReadMemory          = 13,
    //ReadString          = 14,
    //WriteMemory         = 15,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub payload: Payload,
}

impl Message {
    pub fn new(header: Header, payload: Payload) -> Self {
        Self { header, payload }
    }
}


#[derive(Copy, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Header {
    //#[serde(rename="cmd")]
    pub command: Command,
    pub size: usize,
    //pid: i32,
}

impl Header {
    pub fn new(command: Command, size: usize) -> Self {
        Self { command, size }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub payload: Option<Vec<u8>>,
}

impl Payload {
    pub fn new(payload: Option<Vec<u8>>) -> Self{
        Self { payload }
    }

    pub fn len(&self) -> usize {
        match &self.payload {
            Some(x) => x.len(),
            None => 0,
        }
    }
}
