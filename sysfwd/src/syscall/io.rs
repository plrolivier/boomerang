/*
 * 
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Size, Offset, Flag, Buffer, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    tracer::encoder::{ EncodeArg, EncodeEntry },
    operation::Operation,
};


// ssize_t read(int fd, void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Read{
    pub fd: Fd,
    pub buf: Buffer,
    pub count: Size,
    pub retval: Option<Size>,
}
impl Read {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::In, raw.args[2]);
        let count = Size::new(raw.args[2]);
        let retval = None;
        Self { fd, buf, count, retval }
    }
}
impl DecodeEntry for Read {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.buf.decode(pid, operation).unwrap();
        self.count.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Read {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.fd.value;
        raw.args[1] = self.buf.address;
        self.buf.encode(pid, operation).unwrap();
        raw.args[2] = self.count.value;
        Ok(raw)
    }
}

// ssize_t write(int fd, const void buf[.count], size_t count)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Write{
    pub fd: Fd,
    pub buf: Buffer,
    pub count: Size,
    pub retval: Option<Size>,
}
impl Write {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::Out, raw.args[2]);
        let count = Size::new(raw.args[2]);
        let retval = None;
        Self { fd, buf, count, retval }
    }
}
impl DecodeEntry for Write {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.buf.decode(pid, operation).unwrap();
        self.count.decode(pid, operation).unwrap();
    }
}
impl EncodeEntry for Write {
    fn encode_entry(&mut self, mut raw: RawSyscall, pid: i32, operation: &Box<Operation>) -> Result<RawSyscall, std::io::Error> {
        raw.args[0] = self.fd.value;
        raw.args[1] = self.buf.address;
        self.buf.encode(pid, operation).unwrap();
        raw.args[2] = self.count.value;
        Ok(raw)
    }
}

// ssize_t readv(int fd, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Readv {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub retval: Option<Size>,
}
impl Readv {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, iov, iovcnt, retval }
    }
}
impl DecodeEntry for Readv {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
    }
}

// ssize_t writev(int fd, const struct iovec *iov, int iovcnt)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Writev {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub retval: Option<Size>,
}
impl Writev {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        let retval = None;
        Self { fd, iov, iovcnt, retval }
    }
}
impl DecodeEntry for Writev {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
    }
}



// ssize_t pread(int fd, void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Pread {
    pub fd: Fd,
    pub buf: Buffer,
    pub nbytes: Size,
    pub offset: Offset,
    pub retval: Option<Size>,
}
impl Pread {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::In, raw.args[2]);
        let nbytes = Size::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let retval = None;
        Self { fd, buf, nbytes, offset, retval }
    }
}
impl DecodeEntry for Pread {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.buf.decode(pid, operation).unwrap();
        self.nbytes.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
    }
}

// ssize_t pwrite(int fd, const void *buf, size_t nbyte, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Pwrite {
    pub fd: Fd,
    pub buf: Buffer,
    pub nbytes: Size,
    pub offset: Offset,
    pub retval: Option<Size>,
}
impl Pwrite {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let buf = Buffer::new(raw.args[1], Direction::Out, raw.args[2]);
        let nbytes = Size::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let retval = None;
        Self { fd, buf, nbytes, offset, retval }
    }
}
impl DecodeEntry for Pwrite {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.buf.decode(pid, operation).unwrap();
        self.nbytes.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
    }
}


// ssize_t preadv(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Preadv {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub retval: Option<Size>,
}
impl Preadv {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let retval = None;
        Self { fd, iov, iovcnt, offset, retval}
    }
}
impl DecodeEntry for Preadv {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
    }
}

// ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Pwritev {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub retval: Option<Size>,
}
impl Pwritev {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::InOut);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let retval = None;
        Self { fd, iov, iovcnt, offset, retval }
    }
}
impl DecodeEntry for Pwritev {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
    }
}



// ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Preadv2 {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub flags: Flag,
    pub retval: Option<Size>,
}
impl Preadv2 {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let flags = Flag::new(raw.args[4]);
        let retval = None;
        Self { fd, iov, iovcnt, offset, flags, retval }
    }
}
impl DecodeEntry for Preadv2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}

// ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Pwritev2 {
    pub fd: Fd,
    pub iov: Struct,
    pub iovcnt: Integer,
    pub offset: Offset,
    pub flags: Flag,
    pub retval: Option<Size>,

}
impl Pwritev2 {
    pub fn new(raw: RawSyscall) -> Self {
        let fd = Fd::new(raw.args[0]);
        let iov = Struct::new(raw.args[1], Direction::In);
        let iovcnt = Integer::new(raw.args[2]);
        let offset = Offset::new(raw.args[3]);
        let flags = Flag::new(raw.args[4]);
        let retval = None;
        Self { fd, iov, iovcnt, offset, flags, retval }
    }
}
impl DecodeEntry for Pwritev2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.fd.decode(pid, operation).unwrap();
        self.iov.decode(pid, operation).unwrap();
        self.iovcnt.decode(pid, operation).unwrap();
        self.offset.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}