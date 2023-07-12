/*
 *
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int statx(int dirfd, const char *restrict pathname, int flags, unsigned int mask, struct statx *restrict statxbuf);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Statx {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub flags: Flag,
    pub mask: Integer,
    pub statxbuf: Struct,
}
impl Statx {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let flags = Flag::new(raw.args[2]);
        let mask = Integer::new(raw.args[3]);
        let statxbuf = Struct::new(raw.args[4], Direction::InOut);
        Self { dirfd, pathname, flags, mask, statxbuf }
    }
}
impl DecodeEntry for Statx {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation);
        self.pathname.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.mask.decode(pid, operation);
        self.statxbuf.decode(pid, operation);
    }
}