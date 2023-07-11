/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Fd, Offset, NullBuffer },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int truncate(const char *path, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Truncate {
    pub path: NullBuffer,
    pub length: Offset,
}

impl Truncate {
    pub fn new(raw: RawSyscall) -> Self {
        let path = NullBuffer::new(raw.args[0], Direction::In);
        let length = Offset::new(raw.args[1]);
        Self { path, length }
    }
}

impl Decode for Truncate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.path.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}


// int ftruncate(int fd, off_t length)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Ftruncate {
    pub fd: Fd,
    pub length: Offset,
}

impl Ftruncate {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let length = Offset::new(raw.args[1]);
        Self { fd, length }
    }
}

impl Decode for Ftruncate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}