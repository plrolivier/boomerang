/*
 *
 */
use serde::{ Serialize, Deserialize };
use decode_derive::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Address },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



 // pid_t syscall(SYS_set_tid_address, int *tidptr)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct SetTidAddress {
    pub tidptr: Address,
    pub retval: Option<Integer>,
}
impl SetTidAddress {
    pub fn new(raw: RawSyscall) -> Self {
        let tidptr = Address::new(raw.args[0], Direction::In);
        let retval = None;
        Self { tidptr, retval }
    }
}
impl DecodeEntry for SetTidAddress {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.tidptr.decode(pid, operation);
    }
}
