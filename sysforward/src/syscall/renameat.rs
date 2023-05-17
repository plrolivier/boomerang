/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, NullBuffer },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};



// int rename(const char *oldpath, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Rename {
    pub args: Vec<ArgType>,
}

impl Rename {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        Self { args: args }
    }
}

impl Decode for Rename {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Renameat {
    pub args: Vec<ArgType>,
}

impl Renameat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[3], Direction::In)));
        Self { args: args }
    }
}

impl Decode for Renameat {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int renameat2(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Renameat2 {
    pub args: Vec<ArgType>,
}

impl Renameat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[3], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[4])));
        Self { args: args }
    }
}

impl Decode for Renameat2 {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}