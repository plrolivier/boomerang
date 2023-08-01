/*
 *
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::Direction,
    syscall::args::{ Integer, Fd, Flag, NullBuffer, Struct },
    syscall::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    targets::operation::Operation,
};



// int statx(int dirfd, const char *restrict pathname, int flags, unsigned int mask, struct statx *restrict statxbuf);
#[derive(Serialize, Deserialize)]
#[derive(DecodeExit)]
#[derive(Clone, Debug)]
pub struct Statx {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub flags: Flag,
    pub mask: Integer,
    pub statxbuf: Struct,
    pub retval: Option<Integer>,
}
impl Statx {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let flags = Flag::new(raw.args[2]);
        let mask = Integer::new(raw.args[3]);
        let statxbuf = Struct::new(raw.args[4], Direction::InOut);
        let retval = None;
        Self { dirfd, pathname, flags, mask, statxbuf, retval }
    }
}
impl DecodeEntry for Statx {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
        self.mask.decode(pid, operation).unwrap();
        self.statxbuf.decode(pid, operation).unwrap();
    }
}