/*
 *
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    tracer::encoder::{ EncodeArg, EncodeEntry },
    operation::Operation,
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
        self.fd.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
        self.whence.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Lseek {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.fd.value;
        raw.args[1] = self.offset.value;
        raw.args[2] = self.whence.value;
        Ok(raw)
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
        self.fd.decode(pid, operation).unwrap();
        self.offset_high.decode(pid, operation).unwrap();
        self.offset_low.decode(pid, operation).unwrap();
        self.result.decode(pid, operation).unwrap();
        self.whence.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Llseek {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.fd.value;
        raw.args[1] = self.offset_high.value;
        raw.args[2] = self.offset_low.value;
        raw.args[3] = self.result.value;
        raw.args[4] = self.whence.value;
        Ok(raw)
    }
}
