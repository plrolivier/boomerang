/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Fd, Offset, NullBuffer },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};



// int truncate(const char *path, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Truncate {
    pub args: Vec<ArgType>,
}

impl Truncate {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Offset(Offset::new(raw.args[1])));
        Self { args: args }
    }
}

impl Decode for Truncate {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int ftruncate(int fd, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Ftruncate {
    pub args: Vec<ArgType>,
}

impl Ftruncate {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Offset(Offset::new(raw.args[1])));
        Self { args: args }
    }
}

impl Decode for Ftruncate {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}