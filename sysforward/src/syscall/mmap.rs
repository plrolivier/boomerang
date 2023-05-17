/*
 * 
 * int brk(void *addr);
 * void *sbrk(intptr_t increment);
 * void *mmap(void addr[.length], size_t length, int prot, int flags, int fd, off_t offset);
 * void *mremap(void old_address[.old_size], size_t old_size, size_t new_size, int flags, ... /* void *new_address */);
 * int munmap(void addr[.length], size_t length);
 * int mprotect(void addr[.len], size_t len, int prot);
 * int madvise(void addr[.length], size_t length, int advice);
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Flag, Address },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};




#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Brk{
    pub args: Vec<ArgType>,
}
impl Brk {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        Self { args: args }
    }
}
impl Decode for Brk {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Sbrk{
    pub args: Vec<ArgType>,
}
impl Sbrk {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        Self { args: args }
    }
}
impl Decode for Sbrk {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mmap{
    pub args: Vec<ArgType>,
}
impl Mmap {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Protection(Protection::new(raw.args[2])));
        args.push(ArgType::Flag(Flag::new(raw.args[3])));
        args.push(ArgType::Fd(Fd::new(raw.args[4])));
        args.push(ArgType::Offset(Offset::new(raw.args[5])));
        Self { args: args }
    }
}
impl Decode for Mmap {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mremap{
    pub args: Vec<ArgType>,
}
impl Mremap {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Size(Size::new(raw.args[2])));
        args.push(ArgType::Flag(Flag::new(raw.args[3])));
        args.push(ArgType::Address(Address::new(raw.args[4], Direction::In)));
        Self { args: args }
    }
}
impl Decode for Mremap {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}



#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Munmap{
    pub args: Vec<ArgType>,
}
impl Munmap {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        Self { args: args }
    }
}
impl Decode for Munmap {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Mprotect{
    pub args: Vec<ArgType>,
}
impl Mprotect {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Protection(Protection::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Mprotect {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Madvise{
    pub args: Vec<ArgType>,
}
impl Madvise {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Address(Address::new(raw.args[0], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[1])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Madvise {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
