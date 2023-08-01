/*
 * Encode a DecodedSyscall into a RawSyscall.
 *
 * TODO
 */


use crate::{
    syscall::{
        RawSyscall,
        decoder::DecodedSyscall,
    },
    targets::operation::Operation,
};




/* The trait implemented by each syscalls */

pub trait EncodeArg {
    fn encode(&mut self, _pid: i32, _operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}

pub trait EncodeEntry {
    fn encode_entry(&mut self, raw: RawSyscall, _pid: i32, _operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        Ok(raw)
    }
}

pub trait EncodeExit {
    fn encode_exit(&mut self, _value: usize, _pid: i32, _operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        Ok(())
    }
}



/* Wrapper for decoded syscall */

impl EncodeEntry for DecodedSyscall {

    fn encode_entry(&mut self, raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        match self {
            DecodedSyscall::Close(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Creat(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Open(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Openat(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Openat2(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Read(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Write(x) => x.encode_entry(raw, pid, operation),
            DecodedSyscall::Lseek(x) => x.encode_entry(raw, pid, operation),
            //DecodedSyscall::(sysforward/src/tracer/decision_handler.rsx) => x.encode_entry(raw, pid, operation),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Encode trait not implemented for this syscall")),
        }
    }
}