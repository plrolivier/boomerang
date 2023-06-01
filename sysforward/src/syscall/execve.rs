/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};


// int execve(const char *pathname, char *const _Nullable argv[], char *const _Nullable envp[])
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Execve {
    pub args: Vec<ArgType>,
}
impl Execve {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Address(Address::new(raw.args[1], Direction::In)));
        args.push(ArgType::Address(Address::new(raw.args[2], Direction::In)));
        Self { args: args }
    }
}
impl Decode for Execve {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int execveat(int dirfd, const char *pathname, char *const _Nullable argv[], char *const _Nullable envp[], int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Execveat {
    pub args: Vec<ArgType>,
}
impl Execveat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Address(Address::new(raw.args[2], Direction::In)));
        args.push(ArgType::Address(Address::new(raw.args[3], Direction::In)));
        args.push(ArgType::Flag(Flag::new(raw.args[4])));
        Self { args: args }
    }
}
impl Decode for Execveat {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}