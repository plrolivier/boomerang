
/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Buffer, Size, Flag },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// ssize_t getrandom(void buf[.buflen], size_t buflen, unsigned int flags);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Getrandom {
    pub buf: Buffer,
    pub buflen: Size,
    pub flags: Flag,
}

impl Getrandom {
    pub fn new(raw: RawSyscall) -> Self {
        let buf = Buffer::new(raw.args[0], Direction::Out, raw.args[1]);
        let buflen = Size::new(raw.args[1]);
        let flags = Flag::new(raw.args[2]);
        Self { buf, buflen, flags }
    }
}

impl DecodeEntry for Getrandom {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.buf.decode(pid, operation);
        self.buflen.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}