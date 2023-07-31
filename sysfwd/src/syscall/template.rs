/* useless file to delete ! */


/*
 *
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};


// 
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct XXX {
    pub args: Vec<ArgType>,
}
impl XXX {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();

        args.push(ArgType::Integer(Integer::new(raw.args[])));
        args.push(ArgType::Fd(Fd::new(raw.args[])));
        args.push(ArgType::Size(Size::new(raw.args[])));
        args.push(ArgType::Protection(Protection::new(raw.args[])));
        args.push(ArgType::Flag(Flag::new(raw.args[])));
        args.push(ArgType::Offset(Offset::new(raw.args[])));
        args.push(ArgType::Address(Address::new(raw.args[], Direction::In)));
        args.push(ArgType::Buffer(Buffer::new(raw.args[], Direction::Out, raw.args[])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[], Direction::In)));
        Self { args: args }
    }
}
impl Decode for XXX {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

 */