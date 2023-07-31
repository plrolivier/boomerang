/*
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Flag, NullBuffer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::Operation,
};



// int memfd_create(const char *name, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct MemfdCreate {
    pub name: NullBuffer,
    pub flags: Flag,
    pub retval: Option<Integer>,
}

impl MemfdCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let name = NullBuffer::new(raw.args[0], Direction::In);
        let flags = Flag::new(raw.args[1]);
        let retval = None;
        Self { name, flags, retval }
    }
}

impl DecodeEntry for MemfdCreate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.name.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}