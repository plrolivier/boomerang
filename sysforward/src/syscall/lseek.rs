/*
 *
 */
use serde::{ Serialize, Deserialize };

use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// off_t lseek(int fd, off_t offset, int whence);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Lseek {
    pub fd: Fd,
    pub offset: Offset,
    pub whence: Integer,
    pub retval: Option<Offset>,
}
impl Lseek {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let offset = Offset::new(raw.args[1]);
        let whence = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, offset, whence, retval }
    }
}
impl DecodeEntry for Lseek {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.offset.decode(pid, operation);
        self.whence.decode(pid, operation);
    }
}


// int syscall(SYS__llseek, unsigned int fd, unsigned long offset_high, unsigned long offset_low, loff_t *result, unsigned int whence);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Llseek {
    pub fd: Fd,
    pub offset_high: Offset,
    pub offset_low: Offset,
    pub result: Address,
    pub whence: Integer,
    pub retval: Option<Offset>,
}
impl Llseek {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let offset_high = Offset::new(raw.args[1]);
        let offset_low = Offset::new(raw.args[2]);
        let result = Address::new(raw.args[3], Direction::InOut);
        let whence = Integer::new(raw.args[4]);
        let retval = None;
        Self { fd, offset_high, offset_low, result, whence, retval }
    }
}
impl DecodeEntry for Llseek {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.offset_high.decode(pid, operation);
        self.offset_low.decode(pid, operation);
        self.result.decode(pid, operation);
        self.whence.decode(pid, operation);
    }
}
