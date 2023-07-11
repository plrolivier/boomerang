
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

// int ioctl(int fildes, int request, ... /* arg */)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Ioctl {
    pub args: Vec<ArgType>,
}
impl Ioctl {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        // TODO:
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Ioctl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
