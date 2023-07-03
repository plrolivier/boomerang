
/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Buffer, Size, Flag },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};



// ssize_t getrandom(void buf[.buflen], size_t buflen, unsigned int flags);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getrandom {
    pub args: Vec<ArgType>,
}

impl Getrandom {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Buffer(Buffer::new(raw.args[0], Direction::Out, raw.args[1])));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Flag(Flag::new(raw.args[2])));
        Self { args: args }
    }
}

impl Decode for Getrandom {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}