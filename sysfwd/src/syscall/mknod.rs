/*
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, NullBuffer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// int mknod(const char *pathname, mode_t mode, dev_t dev)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Mknod {
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub dev: Integer,
    pub retval: Option<Integer>,
}
impl Mknod {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let mode = Integer::new(raw.args[1]);
        let dev = Integer::new(raw.args[2]);
        let retval = None;
        Self { pathname, mode, dev, retval }
    }
}
impl DecodeEntry for Mknod {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation);
        self.mode.decode(pid, operation);
        self.dev.decode(pid, operation);
    }
}

// int mknodat(int dirfd, const char *pathname, mode_t mode, dev_t dev)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Mknodat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub dev: Integer,
    pub retval: Option<Integer>,
}

impl Mknodat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let mode = Integer::new(raw.args[2]);
        let dev = Integer::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, mode, dev, retval }
    }
}

impl DecodeEntry for Mknodat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.mode.decode(pid, operation);
        self.dev.decode(pid, operation);
    }
}