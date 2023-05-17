/*
 * 
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Fd, Size, Buffer },
    tracer_engine::decoder::{ Decode },
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
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
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
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
