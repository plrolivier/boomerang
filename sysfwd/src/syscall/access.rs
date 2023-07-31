/*
 *
 */
use serde::{ Serialize, Deserialize };
use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Flag, NullBuffer },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::Operation,
};



// int access(const char *pathname, int mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Access {
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub retval: Option<Integer>,
}
impl Access {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let mode = Integer::new(raw.args[1]);
        let retval = None;
        Self { pathname, mode, retval }
    }
}
impl DecodeEntry for Access {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
    }
}


// int faccessat(int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Faccessat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl Faccessat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let mode = Integer::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, mode, flags, retval }
    }
}
impl DecodeEntry for Faccessat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}


// int syscall(SYS_faccessat2, int dirfd, const char *pathname, int mode, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Faccessat2 {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl Faccessat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let mode = Integer::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, mode, flags, retval }
    }
}
impl DecodeEntry for Faccessat2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}
