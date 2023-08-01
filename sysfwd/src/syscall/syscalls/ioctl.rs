
/*
 *
 */
use serde::{ Serialize, Deserialize };
use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    //syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd },
    syscall::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    targets::operation::Operation,
};

// int ioctl(int fildes, int request, ... /* arg */)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Ioctl {
    pub fd: Fd,
    pub request: Integer,
    // TODO:
    pub arg: Integer,
    pub retval: Option<Integer>
}
impl Ioctl {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let request = Integer::new(raw.args[1]);
        // TODO:
        let arg = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, request, arg, retval }
    }
}
impl DecodeEntry for Ioctl {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.request.decode(pid, operation).unwrap();
        self.arg.decode(pid, operation).unwrap();
    }
}
