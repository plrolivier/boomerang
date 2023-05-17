/*
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Flag, Address, NullBuffer, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};



// int name_to_handle_at(int dirfd, const char *pathname, struct file_handle *handle, int *mount_id, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct NameToHandleAt {
    pub args: Vec<ArgType>,
}
impl NameToHandleAt {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[2], Direction::InOut)));
        args.push(ArgType::Address(Address::new(raw.args[3], Direction::InOut)));
        args.push(ArgType::Flag(Flag::new(raw.args[4])));
        Self { args: args }
    }
}
impl Decode for NameToHandleAt {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int open_by_handle_at(int mount_fd, struct file_handle *handle, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct OpenByHandleAt {
    pub args: Vec<ArgType>,
}
impl OpenByHandleAt {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Flag(Flag::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for OpenByHandleAt {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}