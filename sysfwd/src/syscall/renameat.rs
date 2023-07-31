/*
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, NullBuffer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::Operation,
};

use super::args::Flag;



// int rename(const char *oldpath, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Rename {
    pub oldpath: NullBuffer,
    pub newpath: NullBuffer,
    pub retval: Option<Integer>,
}

impl Rename {
    pub fn new(raw: RawSyscall) -> Self {
        let oldpath = NullBuffer::new(raw.args[0], Direction::In);
        let newpath = NullBuffer::new(raw.args[1], Direction::In);
        let retval = None;
        Self { oldpath, newpath, retval }
    }
}

impl DecodeEntry for Rename {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.oldpath.decode(pid, operation).unwrap();
        self.newpath.decode(pid, operation).unwrap();
    }
}

// int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Renameat {
    pub olddirfd: Fd,
    pub oldpath: NullBuffer,
    pub newdirfd: Fd,
    pub newpath: NullBuffer,
    pub retval: Option<Integer>,
}

impl Renameat {
    pub fn new(raw: RawSyscall) -> Self {
        let olddirfd = Fd::new(raw.args[0]);
        let oldpath = NullBuffer::new(raw.args[1], Direction::In);
        let newdirfd = Fd::new(raw.args[2]);
        let newpath = NullBuffer::new(raw.args[3], Direction::In);
        let retval = None;
        Self { olddirfd, oldpath, newdirfd, newpath, retval }
    }
}

impl DecodeEntry for Renameat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.olddirfd.decode(pid, operation).unwrap();
        self.oldpath.decode(pid, operation).unwrap();
        self.newdirfd.decode(pid, operation).unwrap();
        self.newpath.decode(pid, operation).unwrap();
    }
}


// int renameat2(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Renameat2 {
    pub olddirfd: Fd,
    pub oldpath: NullBuffer,
    pub newdirfd: Fd,
    pub newpath: NullBuffer,
    pub flags: Flag,
    pub retval: Option<Integer>,
}

impl Renameat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let olddirfd = Fd::new(raw.args[0]);
        let oldpath = NullBuffer::new(raw.args[1], Direction::In);
        let newdirfd = Fd::new(raw.args[2]);
        let newpath = NullBuffer::new(raw.args[3], Direction::In);
        let flags = Flag::new(raw.args[4]);
        let retval = None;
        Self { olddirfd, oldpath, newdirfd, newpath, flags, retval }
    }
}

impl DecodeEntry for Renameat2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.olddirfd.decode(pid, operation).unwrap();
        self.oldpath.decode(pid, operation).unwrap();
        self.newdirfd.decode(pid, operation).unwrap();
        self.newpath.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}