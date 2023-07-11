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


// int stat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Stat {
    pub args: Vec<ArgType>,
}
impl Stat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Stat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int fstat(int fd, struct stat *statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fstat {
    pub args: Vec<ArgType>,
}
impl Fstat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Fstat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int lstat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Lstat {
    pub args: Vec<ArgType>,
}
impl Lstat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Lstat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


//  int fstatat(int dirfd, const char *restrict pathname, struct stat *restrict statbuf, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fstatat {
    pub args: Vec<ArgType>,
}
impl Fstatat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[2], Direction::InOut)));
        args.push(ArgType::Flag(Flag::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Fstatat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
