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


// int getrlimit(int resource, struct rlimit *rlim)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getrlimit {
    pub args: Vec<ArgType>,
}
impl Getrlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::Out)));
        Self { args: args }
    }
}
impl Decode for Getrlimit {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int setrlimit(int resource, const struct rlimit *rlim)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Setrlimit {
    pub args: Vec<ArgType>,
}
impl Setrlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::In)));
        Self { args: args }
    }
}
impl Decode for Setrlimit {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int prlimit(pid_t pid, int resource, const struct rlimit *_Nullable new_limit, struct rlimit *_Nullable old_limit)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Prlimit {
    pub args: Vec<ArgType>,
}
impl Prlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        args.push(ArgType::Struct(Struct::new(raw.args[2], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[3], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Prlimit {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


// int getrusage(int who, struct rusage *usage)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getrusage {
    pub args: Vec<ArgType>,
}
impl Getrusage {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for Getrusage {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
