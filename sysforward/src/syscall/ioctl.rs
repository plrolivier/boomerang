
/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};

// int ioctl(int fildes, int request, ... /* arg */)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Ioctl {
    pub fd: Fd,
    pub request: Integer,
    // TODO:
    pub arg: Integer,
}
impl Ioctl {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let request = Integer::new(raw.args[1]);
        // TODO:
        let arg = Integer::new(raw.args[2]);
        Self { fd, request, arg }
    }
}
impl Decode for Ioctl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.request.decode(pid, operation);
        self.arg.decode(pid, operation);
    }
}
