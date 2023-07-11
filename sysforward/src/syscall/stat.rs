/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Fd, Flag, NullBuffer, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};


// int stat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Stat {
    pub pathname: NullBuffer,
    pub statbuf: Struct,
}
impl Stat {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        Self { pathname, statbuf }
    }
}
impl Decode for Stat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


// int fstat(int fd, struct stat *statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fstat {
    pub fd: Fd,
    pub statbuf: Struct,
}
impl Fstat {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        Self { fd, statbuf }
    }
}
impl Decode for Fstat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


// int lstat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Lstat {
    pub pathname: NullBuffer,
    pub statbuf: Struct,
}
impl Lstat {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        Self { pathname, statbuf }
    }
}
impl Decode for Lstat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


//  int fstatat(int dirfd, const char *restrict pathname, struct stat *restrict statbuf, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fstatat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub statbuf: Struct,
    pub flags: Flag,
}
impl Fstatat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let statbuf = Struct::new(raw.args[2], Direction::InOut);
        let flags = Flag::new(raw.args[3]);
        Self { dirfd, pathname, statbuf, flags }
    }
}
impl Decode for Fstatat {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}
