/* 
 * Filesystem system calls related
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer_engine::decoder::{ Decode },
    operation::{ Operation },
};


/* File operations */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Close {
    // int close(int fd)
    pub args: Vec<ArgType>,
}
impl Close {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        Self { args: args }
    }
}
impl Decode for Close {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Creat {
    // int creat(const char *pathname, mode_t mode)
    pub args: Vec<ArgType>,
}
impl Creat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        Self { args: args }
    }
}
impl Decode for Creat {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Open {
    // int open(const char *pathname, int flags)
    // int open(const char *pathname, int flags, mode_t mode)
    pub args: Vec<ArgType>,
}
impl Open {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[0], Direction::In)));
        args.push(ArgType::Flag(Flag::new(raw.args[1])));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        Self { args: args }
    }
}
impl Decode for Open {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Openat {
    // int openat(int dirfd, const char *pathname, int flags)
    // int openat(int dirfd, const char *pathname, int flags, mode_t mode)
    pub args: Vec<ArgType>,
}
impl Openat {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Openat {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Openat2 {
    // int openat2(int dirfd, const char *pathname, const struct open_how *how, size_t size)
    pub args: Vec<ArgType>,
}
impl Openat2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::NullBuffer(NullBuffer::new(raw.args[1], Direction::In)));
        args.push(ArgType::Struct(Struct::new(raw.args[2], Direction::In)));
        args.push(ArgType::Size(Size::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Openat2 {
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Read{
    // ssize_t read(int fd, void buf[.count], size_t count)
    pub args: Vec<ArgType>,
}
impl Read {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::In, raw.args[3])));
        args.push(ArgType::Size(Size::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Read {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Write{
    // ssize_t write(int fd, const void buf[.count], size_t count)
    pub args: Vec<ArgType>,
}
impl Write {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Buffer(Buffer::new(raw.args[1], Direction::Out, raw.args[3])));
        args.push(ArgType::Size(Size::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for Write {
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

/* Continue...
            "name_to_handle_at" => {
                // int name_to_handle_at(int dirfd, const char *pathname, struct file_handle *handle, int *mount_id, int flags)
                define_arg!(0, Int);
                define_arg!(1, NullBuf);
                define_arg!(2, Struct);
                define_arg!(3, Address);
                define_arg!(4, Flag);
            },
            "open_by_handle_at" => {
                // int open_by_handle_at(int mount_fd, struct file_handle *handle, int flags)
                define_arg!(0, Int);
                define_arg!(1, Struct);
                define_arg!(2, Flag);
            },
            "memfd_create" => {
                // int memfd_create(const char *name, unsigned int flags)
                syscall.args[0] = Some(Box::new(NullBuf::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(Flag::new(syscall.raw.args[1])));
                define_arg!(0, NullBuf);
                define_arg!(1, Flag);
            },
            "mknod" => {
                // int mknod(const char *pathname, mode_t mode, dev_t dev)
                define_arg!(0, NullBuf);
                define_arg!(1, Int);
                define_arg!(2, Int);
            },
            "mknodat" => {
                // int mknodat(int dirfd, const char *pathname, mode_t mode, dev_t dev)
                define_arg!(0, Int);
                define_arg!(1, NullBuf);
                define_arg!(2, Int);
                define_arg!(3, Int);
            },
            "rename" => {
                // int rename(const char *oldpath, const char *newpath)
                define_arg!(0, NullBuf);
                define_arg!(1, NullBuf);
            },
            "renameat" => {
                // int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath)
                define_arg!(0, Int);
                define_arg!(1, NullBuf);
                define_arg!(2, Int);
                define_arg!(3, NullBuf);
            },
            "renameat2" => {
                // int renameat2(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, unsigned int flags)
                define_arg!(0, Int);
                define_arg!(1, NullBuf);
                define_arg!(2, Int);
                define_arg!(3, NullBuf);
                define_arg!(4, Int);
            },
            "truncate" => {
                // int truncate(const char *path, off_t length)
                define_arg!(0, NullBuf);
                define_arg!(1, Offset);
            },
            "ftruncate" => {
                // int ftruncate(int fd, off_t length)
                define_arg!(0, Fd);
                define_arg!(1, Offset);
            },
            "fallocate" => {
                // int fallocate(int fd, int mode, off_t offset, off_t len)
                define_arg!(0, Fd);
                define_arg!(1, Int);
                define_arg!(2, Offset);
                define_arg!(3, Offset);
            },
*/
