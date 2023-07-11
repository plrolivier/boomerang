/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int access(const char *pathname, int mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Access {
    pub args: Vec<ArgType>,
}
impl Access {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        Self { args: args }
    }
}
impl Decode for Access {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int faccessat(int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Faccessat {
    pub args: Vec<ArgType>,
}
impl Faccessat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Flag(Flag::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Faccessat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int syscall(SYS_faccessat2, int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Faccessat2 {
    pub args: Vec<ArgType>,
}
impl Faccessat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Flag(Flag::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Faccessat2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
