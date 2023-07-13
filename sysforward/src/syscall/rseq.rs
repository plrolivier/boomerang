/*
 */
use serde::{ Serialize, Deserialize };

use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Size, Flag, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// int syscall(SYS_rseq, struct rseq *rseq, uint32_t rseq_len, int flags, uint32_t sig)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Rseq {
    pub rseq: Struct,
    pub rseq_len: Size,
    pub flags: Flag,
    pub sig: Integer,
    pub retval: Option<Integer>,
}
impl Rseq {
    pub fn new(raw: RawSyscall) -> Self {
        let rseq = Struct::new(raw.args[0], Direction::InOut);
        let rseq_len = Size::new(raw.args[1]);
        let flags = Flag::new(raw.args[2]);
        let sig = Integer::new(raw.args[3]);
        let retval = None;
        Self { rseq, rseq_len, flags, sig, retval }
    }
}
impl DecodeEntry for Rseq {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.rseq.decode(pid, operation);
        self.rseq_len.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.sig.decode(pid, operation);
    }
}
