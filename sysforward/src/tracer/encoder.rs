/*
 * Encode a DecodedSyscall into a RawSyscall.
 *
 * TODO
 */


use crate::{
    arch::{ Architecture },
    syscall::{ self, Syscall, RawSyscall },
    operation::Operation,
    tracer::decoder::{ DecodedSyscall },
};




/* The trait implemented by each syscalls */

pub trait EncodeArg {
    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}

pub trait EncodeEntry {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        Ok(raw)
    }
}

pub trait EncodeExit {
    fn encode_exit(&mut self, value: usize, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}



/* Wrapper for decoded syscall */

impl EncodeEntry for DecodedSyscall {

    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        match self {
            DecodedSyscall::Close(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Creat(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Open(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Openat(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Openat2(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Read(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Write(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Lseek(x) => x.encode_entry(raw, pid, operation),
            //DecodedSyscall::(x) => x.encode_entry(raw, pid, operation),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Encode trait not implemented for this syscall")),
        }
    }
}