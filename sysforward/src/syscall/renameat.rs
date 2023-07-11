/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, NullBuffer },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};

use super::args::Flag;



// int rename(const char *oldpath, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Rename {
    pub oldpath: NullBuffer,
    pub newpath: NullBuffer,
}

impl Rename {
    pub fn new(raw: RawSyscall) -> Self {
        let oldpath = NullBuffer::new(raw.args[0], Direction::In);
        let newpath = NullBuffer::new(raw.args[1], Direction::In);
        Self { oldpath, newpath }
    }
}

impl Decode for Rename {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.oldpath.decode(pid, operation);
        self.newpath.decode(pid, operation);
    }
}

// int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Renameat {
    pub olddirfd: Fd,
    pub oldpath: NullBuffer,
    pub newdirfd: Fd,
    pub newpath: NullBuffer,
}

impl Renameat {
    pub fn new(raw: RawSyscall) -> Self {
        let olddirfd = Fd::new(raw.args[0]);
        let oldpath = NullBuffer::new(raw.args[1], Direction::In);
        let newdirfd = Fd::new(raw.args[2]);
        let newpath = NullBuffer::new(raw.args[3], Direction::In);
        Self { olddirfd, oldpath, newdirfd, newpath }
    }
}

impl Decode for Renameat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.olddirfd.decode(pid, operation);
        self.oldpath.decode(pid, operation);
        self.newdirfd.decode(pid, operation);
        self.newpath.decode(pid, operation);
    }
}


// int renameat2(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Renameat2 {
    pub olddirfd: Fd,
    pub oldpath: NullBuffer,
    pub newdirfd: Fd,
    pub newpath: NullBuffer,
    pub flags: Flag,
}

impl Renameat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let olddirfd = Fd::new(raw.args[0]);
        let oldpath = NullBuffer::new(raw.args[1], Direction::In);
        let newdirfd = Fd::new(raw.args[2]);
        let newpath = NullBuffer::new(raw.args[3], Direction::In);
        let flags = Flag::new(raw.args[4]);
        Self { olddirfd, oldpath, newdirfd, newpath, flags }
    }
}

impl Decode for Renameat2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.olddirfd.decode(pid, operation);
        self.oldpath.decode(pid, operation);
        self.newdirfd.decode(pid, operation);
        self.newpath.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}