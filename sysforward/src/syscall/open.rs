/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Flag, NullBuffer, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};


// int close(int fd)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Close {
    pub args: Vec<ArgType>,
}
impl Close {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        Self { args: args }
    }
}
impl Decode for Close {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int creat(const char *pathname, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Creat {
    pub args: Vec<ArgType>,
}
impl Creat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        Self { args: args }
    }
}
impl Decode for Creat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int open(const char *pathname, int flags)
// int open(const char *pathname, int flags, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Open {
    pub args: Vec<ArgType>,
}
impl Open {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Flag(Flag::new(raw.args[1])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Open {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int openat(int dirfd, const char *pathname, int flags)
// int openat(int dirfd, const char *pathname, int flags, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Openat {
    pub args: Vec<ArgType>,
}
impl Openat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Openat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int openat2(int dirfd, const char *pathname, const struct open_how *how, size_t size)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Openat2 {
    pub args: Vec<ArgType>,
}
impl Openat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[2], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Openat2 {
}