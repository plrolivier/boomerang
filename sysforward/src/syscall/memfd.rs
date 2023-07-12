/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Flag, NullBuffer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int memfd_create(const char *name, unsigned int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct MemfdCreate {
    pub name: NullBuffer,
    pub flags: Flag,
}

impl MemfdCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let name = NullBuffer::new(raw.args[0], Direction::In);
        let flags = Flag::new(raw.args[1]);
        Self { name, flags }
    }
}

impl DecodeEntry for MemfdCreate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.name.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}