
/*
 *
 */
use serde::{ Serialize, Deserialize };

//use decoding_macro::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Integer },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// [[noreturn]] void syscall(SYS_exit_group, int status);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
//#[derive(DecodeExit)]
pub struct ExitGroup {
    pub status: Integer,
    pub retval: Option<Integer>,
}
impl ExitGroup {
    pub fn new(raw: RawSyscall) -> Self {
        let status = Integer::new(raw.args[0]);
        let retval = None;
        Self { status, retval }
    }
}
impl DecodeEntry for ExitGroup {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.status.decode(pid, operation);
    }
}
