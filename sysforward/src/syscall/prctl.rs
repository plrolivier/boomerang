/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};


// int prctl(int option, unsigned long arg2, unsigned long arg3, unsigned long arg4, unsigned long arg5)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Prctl {
    pub args: Vec<ArgType>,
}
impl Prctl {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        args.push(ArgType::Integer(Integer::new(raw.args[4])));
        Self { args: args }
    }
}
impl Decode for Prctl {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int syscall(SYS_arch_prctl, int code, unsigned long addr)
// int syscall(SYS_arch_prctl, int code, unsigned long *addr)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct ArchPrctl {
    pub args: Vec<ArgType>,
}
impl ArchPrctl {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Address(Address::new(raw.args[1], Direction::In)));
        Self { args: args }
    }
}
impl Decode for ArchPrctl {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}