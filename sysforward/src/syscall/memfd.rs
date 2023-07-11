/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Flag, NullBuffer },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int memfd_create(const char *name, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct MemfdCreate {
    pub args: Vec<ArgType>,
}

impl MemfdCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Flag(Flag::new(raw.args[1])));
        Self { args: args }
    }
}

impl Decode for MemfdCreate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}