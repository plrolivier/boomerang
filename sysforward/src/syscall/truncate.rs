/*
 */
use serde::{ Serialize, Deserialize };

use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Offset, NullBuffer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int truncate(const char *path, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Truncate {
    pub path: NullBuffer,
    pub length: Offset,
    pub retval: Option<Integer>,
}

impl Truncate {
    pub fn new(raw: RawSyscall) -> Self {
        let path = NullBuffer::new(raw.args[0], Direction::In);
        let length = Offset::new(raw.args[1]);
        let retval = None;
        Self { path, length, retval }
    }
}

impl DecodeEntry for Truncate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.path.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}


// int ftruncate(int fd, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Ftruncate {
    pub fd: Fd,
    pub length: Offset,
    pub retval: Option<Integer>,
}

impl Ftruncate {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let length = Offset::new(raw.args[1]);
        let retval = None;
        Self { fd, length, retval }
    }
}

impl DecodeEntry for Ftruncate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}