/*
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Integer, Fd, Offset},
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::Operation,
};



// int fallocate(int fd, int mode, off_t offset, off_t len)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Fallocate {
    pub fd: Fd,
    pub mode: Integer,
    pub offset: Offset,
    pub len: Offset,
    pub retval: Option<Integer>,
}

impl Fallocate {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let mode = Integer::new(raw.args[1]);
        let offset = Offset::new(raw.args[2]);
        let len = Offset::new(raw.args[3]);
        let retval = None;
        Self { fd, mode, offset, len, retval }
    }
}

impl DecodeEntry for Fallocate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
        self.len.decode(pid, operation).unwrap();
    }
}