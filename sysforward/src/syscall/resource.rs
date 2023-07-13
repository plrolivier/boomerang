use nix::sys::resource;
/*
 *
 */
use serde::{ Serialize, Deserialize };
use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// int getrlimit(int resource, struct rlimit *rlim)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Getrlimit {
    pub resource: Integer,
    pub rlim: Struct,
    pub retval: Option<Integer>,
}
impl Getrlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let resource = Integer::new(raw.args[0]);
        let rlim = Struct::new(raw.args[1], Direction::Out);
        let retval = None;
        Self { resource, rlim, retval }
    }
}
impl DecodeEntry for Getrlimit {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.resource.decode(pid, operation);
        self.rlim.decode(pid, operation);
    }
}


// int setrlimit(int resource, const struct rlimit *rlim)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Setrlimit {
    pub resource: Integer,
    pub rlim: Struct,
    pub retval: Option<Integer>,
}
impl Setrlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let resource = Integer::new(raw.args[0]);
        let rlim = Struct::new(raw.args[1], Direction::In);
        let retval = None;
        Self { resource, rlim, retval }
    }
}
impl DecodeEntry for Setrlimit {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.resource.decode(pid, operation);
        self.rlim.decode(pid, operation);
    }
}


// int prlimit(pid_t pid, int resource, const struct rlimit *_Nullable new_limit, struct rlimit *_Nullable old_limit)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Prlimit {
    pub pid: Integer,
    pub resource: Integer,
    pub new_limit: Struct,
    pub old_limit: Struct,
    pub retval: Option<Integer>,
}
impl Prlimit {
    pub fn new(raw: RawSyscall) -> Self {
        let pid = Integer::new(raw.args[0]);
        let resource = Integer::new(raw.args[1]);
        let new_limit = Struct::new(raw.args[2], Direction::In);
        let old_limit = Struct::new(raw.args[3], Direction::InOut);
        let retval = None;
        Self { pid, resource, new_limit, old_limit, retval }
    }
}
impl DecodeEntry for Prlimit {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pid.decode(pid, operation);
        self.resource.decode(pid, operation);
        self.new_limit.decode(pid, operation);
        self.old_limit.decode(pid, operation);
    }
}


// int getrusage(int who, struct rusage *usage)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Getrusage {
    pub who: Integer,
    pub usage: Struct,
    pub retval: Option<Integer>,
}
impl Getrusage {
    pub fn new(raw: RawSyscall) -> Self {
        let who = Integer::new(raw.args[0]);
        let usage = Struct::new(raw.args[1], Direction::InOut);
        let retval = None;
        Self { who, usage, retval }
    }
}
impl DecodeEntry for Getrusage {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.who.decode(pid, operation);
        self.usage.decode(pid, operation);
    }
}
