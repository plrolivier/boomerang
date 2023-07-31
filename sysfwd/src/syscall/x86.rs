/*
 *
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Address },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int syscall(SYS_arch_prctl, int code, unsigned long addr);
// int syscall(SYS_arch_prctl, int code, unsigned long *addr);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct ArchPrctl {
    pub code: Integer,
    pub addr: Address,
    pub retval: Option<Integer>,
}
impl ArchPrctl {
    pub fn new(raw: RawSyscall) -> Self {
        let code = Integer::new(raw.args[0]);
        let addr = Address::new(raw.args[1], Direction::In);
        let retval = None;
        Self { code, addr, retval }
    }
}
impl DecodeEntry for ArchPrctl {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.code.decode(pid, operation);
        self.addr.decode(pid, operation);
    }
}
*/
