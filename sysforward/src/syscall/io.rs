/*
 * 
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};


// ssize_t read(int fd, void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Read{
    pub args: Vec<ArgType>,
}
impl Read {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::In, raw.args[2])));
        args.push(ArgType::Size(Size::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Read {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t write(int fd, const void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Write{
    pub args: Vec<ArgType>,
}
impl Write {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::Out, raw.args[2])));
        args.push(ArgType::Size(Size::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Write {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t readv(int fildes, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Readv {
    pub args: Vec<ArgType>,
}
impl Readv {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Readv {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t writev(int fildes, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Writev {
    pub args: Vec<ArgType>,
}
impl Writev {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Writev {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}



// ssize_t pread(int fildes, void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pread {
    pub args: Vec<ArgType>,
}
impl Pread {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::In, raw.args[2])));
        args.push(ArgType::Size(Size::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Pread {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t pwrite(int fildes, const void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwrite {
    pub args: Vec<ArgType>,
}
impl Pwrite {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::Out, raw.args[2])));
        args.push(ArgType::Size(Size::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Pwrite {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// ssize_t preadv(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Preadv {
    pub args: Vec<ArgType>,
}
impl Preadv {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Preadv {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwritev {
    pub args: Vec<ArgType>,
}
impl Pwritev {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Pwritev {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}



// ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Preadv2 {
    pub args: Vec<ArgType>,
}
impl Preadv2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        args.push(ArgType::Flag(Flag::new(raw.args[4])));
        Self { args: args }
    }
}
impl Decode for Preadv2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwritev2 {
    pub args: Vec<ArgType>,
}
impl Pwritev2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        args.push(ArgType::Flag(Flag::new(raw.args[4])));
        Self { args: args }
    }
}
impl Decode for Pwritev2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}