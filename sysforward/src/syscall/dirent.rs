/*
 *
 */
use serde::{ Serialize, Deserialize };
use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// long syscall(SYS_getdents, unsigned int fd, struct linux_dirent *dirp, unsigned int count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Getdents {
    pub fd: Fd,
    pub dirp: Struct,
    pub count: Integer,
    pub retval: Option<Integer>,
}
impl Getdents {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let dirp = Struct::new(raw.args[1], Direction::In);
        let count = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, dirp, count, retval }
    }
}
impl DecodeEntry for Getdents {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.dirp.decode(pid, operation);
        self.count.decode(pid, operation);
    }
}


// ssize_t getdents64(int fd, void dirp[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Getdents64 {
    pub fd: Fd,
    pub dirp: Struct,
    pub count: Integer,
    pub retval: Option<Size>,
}
impl Getdents64 {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let dirp = Struct::new(raw.args[1], Direction::In);
        let count = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, dirp, count, retval }
    }
}
impl DecodeEntry for Getdents64 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.dirp.decode(pid, operation);
        self.count.decode(pid, operation);
    }
}


// int syscall(SYS_readdir, unsigned int fd, struct old_linux_dirent *dirp, unsigned int count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Readdir {
    pub fd: Fd,
    pub dirp: Struct,
    pub count: Integer,
    pub retval: Option<Integer>,
}
impl Readdir {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let dirp = Struct::new(raw.args[1], Direction::In);
        let count = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, dirp, count, retval }
    }
}
impl DecodeEntry for Readdir {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.dirp.decode(pid, operation);
        self.count.decode(pid, operation);
    }
}
