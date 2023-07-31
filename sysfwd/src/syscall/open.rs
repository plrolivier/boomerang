/*
 *
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;

use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Size, Flag, NullBuffer, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    tracer::encoder::{ EncodeEntry, EncodeExit, EncodeArg },
    operation::Operation,
};


// int close(int fd)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Close {
    pub fd: Fd,
    pub retval: Option<Integer>,
}
impl Close {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let retval = None;
        Self { fd, retval }
    }
}
impl DecodeEntry for Close {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Close {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.fd.value;
        Ok(raw)
    }
}

// int creat(const char *pathname, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Creat {
    pub pathname: NullBuffer,
    pub mode: Integer,
    pub retval: Option<Fd>,
}
impl Creat {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let mode = Integer::new(raw.args[1]);
        let retval = None;
        Self { pathname, mode, retval }
    }
}
impl DecodeEntry for Creat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Creat {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.pathname.address;
        self.pathname.encode(pid, operation).unwrap();
        raw.args[1] = self.mode.value;
        Ok(raw)
    }
}

// int open(const char *pathname, int flags)
// int open(const char *pathname, int flags, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Open {
    pub pathname: NullBuffer,
    pub flags: Flag,
    pub mode: Integer,
    pub retval: Option<Fd>,
}
impl Open {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let flags = Flag::new(raw.args[1]);
        let mode = Integer::new(raw.args[2]);
        let retval = None;
        Self { pathname, flags, mode, retval }
    }
}
impl DecodeEntry for Open {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Open {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.pathname.address;
        self.pathname.encode(pid, operation).unwrap();
        raw.args[1] = self.mode.value;
        Ok(raw)
    }
}
/* 
impl DecodeExit for Open {
    fn decode_exit(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        self.retval.as_mut().unwrap().decode(pid, operation).unwrap();
        Ok(())
    }
}
*/

// int openat(int dirfd, const char *pathname, int flags)
// int openat(int dirfd, const char *pathname, int flags, mode_t mode)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Openat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub flags: Integer,
    pub mode: Integer,
    pub retval: Option<Fd>,
}
impl Openat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let flags = Integer::new(raw.args[2]);
        let mode = Integer::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, flags, mode, retval }
    }
}
impl DecodeEntry for Openat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
        self.mode.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Openat {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.dirfd.value;
        raw.args[1] = self.pathname.address;
        self.pathname.encode(pid, operation).unwrap();
        raw.args[2] = self.flags.value;
        raw.args[3] = self.mode.value;
        Ok(raw)
    }
}

// int openat2(int dirfd, const char *pathname, const struct open_how *how, size_t size)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Openat2 {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub how: Struct,
    pub size: Size,
    pub retval: Option<Fd>,
}
impl Openat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[1], Direction::In);
        let how = Struct::new(raw.args[2], Direction::In);
        let size = Size::new(raw.args[3]);
        let retval = None;
        Self { dirfd, pathname, how, size, retval }
    }
}
impl DecodeEntry for Openat2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.how.decode(pid, operation).unwrap();
        self.size.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Openat2 {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.dirfd.value;
        raw.args[1] = self.pathname.address;
        self.pathname.encode(pid, operation).unwrap();
        raw.args[2] = self.how.address;
        self.how.encode(pid, operation).unwrap();
        raw.args[3] = self.size.value;
        Ok(raw)
    }
}
