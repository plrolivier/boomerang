/*
 *
 */
use serde::{ Serialize, Deserialize };
use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Flag, NullBuffer, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// int stat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Stat {
    pub pathname: NullBuffer,
    pub statbuf: Struct,
    pub retval: Option<Integer>,
}
impl Stat {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        let retval = None;
        Self { pathname, statbuf, retval }
    }
}
impl DecodeEntry for Stat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


// int fstat(int fd, struct stat *statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Fstat {
    pub fd: Fd,
    pub statbuf: Struct,
    pub retval: Option<Integer>,
}
impl Fstat {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        let retval = None;
        Self { fd, statbuf, retval }
    }
}
impl DecodeEntry for Fstat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


// int lstat(const char *restrict pathname, struct stat *restrict statbuf)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Lstat {
    pub pathname: NullBuffer,
    pub statbuf: Struct,
    pub retval: Option<Integer>,
}
impl Lstat {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let statbuf = Struct::new(raw.args[1], Direction::InOut);
        let retval = None;
        Self { pathname, statbuf, retval }
    }
}
impl DecodeEntry for Lstat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
    }
}


//  int fstatat(int dirfd, const char *restrict pathname, struct stat *restrict statbuf, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Fstatat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub statbuf: Struct,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl Fstatat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let statbuf = Struct::new(raw.args[2], Direction::InOut);
        let flags = Flag::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, statbuf, flags, retval }
    }
}
impl DecodeEntry for Fstatat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.statbuf.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}
