/*
 * 
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::{ Operation },
};


// ssize_t read(int fd, void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Read{
    pub fd: Fd,
    pub buf: Buffer,
    pub count: Size,
}
impl Read {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::In, raw.args[2]);
        let count = Size::new(raw.args[2]);
        Self { fd, buf, count }
    }
}
impl DecodeEntry for Read {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.buf.decode(pid, operation);
        self.count.decode(pid, operation);
    }
}

// ssize_t write(int fd, const void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Write{
    pub fd: Fd,
    pub buf: Buffer,
    pub count: Size,
}
impl Write {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::Out, raw.args[2]);
        let count = Size::new(raw.args[2]);
        Self { fd, buf, count }
    }
}
impl DecodeEntry for Write {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.buf.decode(pid, operation);
        self.count.decode(pid, operation);
    }
}

// ssize_t readv(int fd, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Readv {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
}
impl Readv {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        Self { fd, iov, iovcnt }
    }
}
impl DecodeEntry for Readv {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
    }
}

// ssize_t writev(int fd, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Writev {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
}
impl Writev {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        Self { fd, iov, iovcnt }
    }
}
impl DecodeEntry for Writev {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
    }
}



// ssize_t pread(int fd, void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pread {
    pub fd: Fd,
    pub buf: Buffer,
    pub nbytes: Size,
    pub offset: Offset,
}
impl Pread {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::In, raw.args[2]);
        let nbytes = Size::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        Self { fd, buf, nbytes, offset }
    }
}
impl DecodeEntry for Pread {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.buf.decode(pid, operation);
        self.nbytes.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}

// ssize_t pwrite(int fd, const void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwrite {
    pub fd: Fd,
    pub buf: Buffer,
    pub nbytes: Size,
    pub offset: Offset,
}
impl Pwrite {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::Out, raw.args[2]);
        let nbytes = Size::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        Self { fd, buf, nbytes, offset }
    }
}
impl DecodeEntry for Pwrite {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.buf.decode(pid, operation);
        self.nbytes.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}


// ssize_t preadv(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Preadv {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
}
impl Preadv {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        Self { fd, iov, iovcnt, offset }
    }
}
impl DecodeEntry for Preadv {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}

// ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwritev {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
}
impl Pwritev {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        Self { fd, iov, iovcnt, offset }
    }
}
impl DecodeEntry for Pwritev {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
        self.offset.decode(pid, operation);
    }
}



// ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Preadv2 {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub flags: Flag,
}
impl Preadv2 {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let flags = Flag::new(raw.args[4]);
        Self { fd, iov, iovcnt, offset, flags }
    }
}
impl DecodeEntry for Preadv2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
        self.offset.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}

// ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Pwritev2 {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub flags: Flag,
}
impl Pwritev2 {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let flags = Flag::new(raw.args[4]);
        Self { fd, iov, iovcnt, offset, flags }
    }
}
impl DecodeEntry for Pwritev2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation);
        self.iov.decode(pid, operation);
        self.iovcnt.decode(pid, operation);
        self.offset.decode(pid, operation);
        self.flags.decode(pid, operation);
    }
}