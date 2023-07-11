/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, NullBuffer },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};


// int mknod(const char *pathname, mode_t mode, dev_t dev)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mknod {
    pub args: Vec<ArgType>,
}
impl Mknod {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Mknod {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int mknodat(int dirfd, const char *pathname, mode_t mode, dev_t dev)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mknodat {
    pub args: Vec<ArgType>,
}

impl Mknodat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        Self { args: args }
    }
}

impl Decode for Mknodat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}