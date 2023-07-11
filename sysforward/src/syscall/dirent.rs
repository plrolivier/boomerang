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


// long syscall(SYS_getdents, unsigned int fd, struct linux_dirent *dirp, unsigned int count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getdents {
    pub args: Vec<ArgType>,
}
impl Getdents {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Getdents {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// ssize_t getdents64(int fd, void dirp[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getdents64 {
    pub args: Vec<ArgType>,
}
impl Getdents64 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::Out, raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Getdents64 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int syscall(SYS_readdir, unsigned int fd, struct old_linux_dirent *dirp, unsigned int count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Readdir {
    pub args: Vec<ArgType>,
}
impl Readdir {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Readdir {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
