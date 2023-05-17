/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType },
    syscall::args::{ Integer, Fd, Offset},
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};



// int fallocate(int fd, int mode, off_t offset, off_t len)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fallocate {
    pub args: Vec<ArgType>,
}

impl Fallocate {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        args.push(ArgType::Offset(Offset::new(raw.args[2])));
        args.push(ArgType::Offset(Offset::new(raw.args[3])));
        Self { args: args }
    }
}

impl Decode for Fallocate {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}