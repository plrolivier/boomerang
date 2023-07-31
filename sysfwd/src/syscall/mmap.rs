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

use decoding_macro::DecodeExit;
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Flag, Address },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};



// int brk(void *addr);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Brk{
    pub addr: Address,
    pub retval: Option<Integer>,
}
impl Brk {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let retval = None;
        Self { addr, retval }
    }
}
impl DecodeEntry for Brk {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
    }
}


// void *sbrk(intptr_t increment);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Sbrk{
    pub increment: Integer,
    pub retval: Option<Address>,
}
impl Sbrk {
    pub fn new(raw: RawSyscall) -> Self {
        let increment = Integer::new(raw.args[0]);
        let retval = None;
        Self { increment, retval }
    }
}
impl DecodeEntry for Sbrk {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.increment.decode(pid, operation);
    }
}


// void *mmap(void addr[.length], size_t length, int prot, int flags, int fd, off_t offset);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Mmap{
    pub addr: Address,
    pub length: Size,
    pub prot: Protection,
    pub flags: Flag,
    pub fd: Fd,
    pub offset: Offset,
    pub retval: Option<Address>,
}
impl Mmap {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        let prot = Protection::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let fd = Fd::new(raw.args[4]);
        let offset = Offset::new(raw.args[5]);
        let retval = None;
        Self { addr, length, prot, flags, fd, offset, retval }
    }
}
impl DecodeEntry for Mmap {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
        self.prot.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.fd.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}


// void *mremap(void old_address[.old_size], size_t old_size, size_t new_size, int flags, ... /* void *new_address */);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Mremap{
    pub old_address: Address,
    pub old_size: Size,
    pub new_size: Size,
    pub flags: Flag,
    pub new_address: Address,
    pub retval: Option<Address>,
}
impl Mremap {
    pub fn new(raw: RawSyscall) -> Self {
        let old_address = Address::new(raw.args[0], Direction::In);
        let old_size = Size::new(raw.args[1]);
        let new_size = Size::new(raw.args[2]);
        let flags = Flag::new(raw.args[3]);
        let new_address = Address::new(raw.args[4], Direction::In);
        let retval = None;
        Self { old_address, old_size, new_size, flags, new_address, retval }
    }
}
impl DecodeEntry for Mremap {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.old_address.decode(pid, operation);
        self.old_size.decode(pid, operation);
        self.new_size.decode(pid, operation);
        self.flags.decode(pid, operation);
        self.new_address.decode(pid, operation);
    }
}



// int munmap(void addr[.length], size_t length);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Munmap{
    pub addr: Address,
    pub length: Size,
    pub retval: Option<Integer>,
}
impl Munmap {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        let retval = None;
        Self { addr, length, retval }
    }
}
impl DecodeEntry for Munmap {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
    }
}


// int mprotect(void addr[.len], size_t len, int prot);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Mprotect{
    pub addr: Address,
    pub len: Size,
    pub prot: Protection,
    pub retval: Option<Integer>,
}
impl Mprotect {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let len = Size::new(raw.args[1]);
        let prot = Protection::new(raw.args[2]);
        let retval = None;
        Self { addr, len, prot, retval }
    }
}
impl DecodeEntry for Mprotect {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.len.decode(pid, operation);
        self.prot.decode(pid, operation);
    }
}


// int madvise(void addr[.length], size_t length, int advice);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Madvise{
    pub addr: Address,
    pub length: Size,
    pub advice: Integer,
    pub retval: Option<Integer>,
}
impl Madvise {
    pub fn new(raw: RawSyscall) -> Self {
        let addr = Address::new(raw.args[0], Direction::In);
        let length = Size::new(raw.args[1]);
        let advice = Integer::new(raw.args[2]);
        let retval = None;
        Self { addr, length, advice, retval }
    }
}
impl DecodeEntry for Madvise {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.addr.decode(pid, operation);
        self.length.decode(pid, operation);
        self.advice.decode(pid, operation);
    }
}
