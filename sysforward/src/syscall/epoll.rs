/*
 *
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int epoll_create(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCreate {
    pub args: Vec<ArgType>,
}
impl EpollCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        Self { args: args }
    }
}
impl Decode for EpollCreate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int epoll_create1(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCreate1 {
    pub args: Vec<ArgType>,
}
impl EpollCreate1 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Integer(Integer::new(raw.args[0])));
        Self { args: args }
    }
}
impl Decode for EpollCreate1 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int epoll_ctl(int epfd, int op, int fd, struct epoll_event *_Nullable event);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCtl {
    pub args: Vec<ArgType>,
}
impl EpollCtl {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Integer(Integer::new(raw.args[1])));
        args.push(ArgType::Fd(Fd::new(raw.args[2])));
        args.push(ArgType::Struct(Struct::new(raw.args[3], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for EpollCtl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollWait {
    pub args: Vec<ArgType>,
}
impl EpollWait {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        Self { args: args }
    }
}
impl Decode for EpollWait {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int epoll_pwait(int epfd, struct epoll_event *events, int maxevents, int timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollPwait {
    pub args: Vec<ArgType>,
}
impl EpollPwait {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Integer(Integer::new(raw.args[3])));
        args.push(ArgType::Struct(Struct::new(raw.args[4], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for EpollPwait {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}

// int epoll_pwait2(int epfd, struct epoll_event *events, int maxevents, const struct timespec *_Nullable timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollPwait2 {
    pub args: Vec<ArgType>,
}
impl EpollPwait2 {
    pub fn new(raw: RawSyscall) -> Self {
        let mut args = Vec::new();
        args.push(ArgType::Fd(Fd::new(raw.args[0])));
        args.push(ArgType::Struct(Struct::new(raw.args[1], Direction::InOut)));
        args.push(ArgType::Integer(Integer::new(raw.args[2])));
        args.push(ArgType::Struct(Struct::new(raw.args[3], Direction::InOut)));
        args.push(ArgType::Struct(Struct::new(raw.args[4], Direction::InOut)));
        Self { args: args }
    }
}
impl Decode for EpollPwait2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.args.iter_mut().for_each(|arg| arg.decode(pid, operation));
    }
}
