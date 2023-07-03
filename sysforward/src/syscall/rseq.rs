/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Size, Flag, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};


// int syscall(SYS_rseq, struct rseq *rseq, uint32_t rseq_len, int flags, uint32_t sig)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Rseq {
    pub args: Vec<ArgType>,
}
impl Rseq {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Struct(Struct::new(raw.args[0], Direction::InOut)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Flag(Flag::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Rseq {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
