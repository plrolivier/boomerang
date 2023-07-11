/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Integer, Fd, Offset},
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int fallocate(int fd, int mode, off_t offset, off_t len)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fallocate {
    pub fd: Fd,
    pub mode: Integer,
    pub offset: Offset,
    pub len: Offset,
}

impl Fallocate {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let mode = Integer::new(raw.args[1]);
        let offset = Offset::new(raw.args[2]);
        let len = Offset::new(raw.args[3]);
        Self { fd, mode, offset, len }
    }
}

impl Decode for Fallocate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.mode.decode(pid, operation);
        self.offset.decode(pid, operation);
        self.len.decode(pid, operation);
    }
}