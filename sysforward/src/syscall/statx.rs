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



// int statx(int dirfd, const char *restrict pathname, int flags, unsigned int mask, struct statx *restrict statxbuf);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Statx {
    pub args: Vec<ArgType>,
}
impl Statx {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        args.push(ArgType::Struct(Struct::new(raw.args[4], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Statx {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}