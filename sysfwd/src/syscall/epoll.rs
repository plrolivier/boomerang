/*
 *
 */
use serde::{ Serialize, Deserialize };

use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Struct },
    tracer::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    operation::Operation,
};



// int epoll_create(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollCreate {
    pub size: Integer,
    pub retval: Option<Integer>,
}
impl EpollCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let size = Integer::new(raw.args[0]);
        let retval = None;
        Self { size, retval }
    }
}
impl DecodeEntry for EpollCreate {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.size.decode(pid, operation).unwrap();
    }
}

// int epoll_create1(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollCreate1 {
    pub size: Integer,
    pub retval: Option<Integer>,
}
impl EpollCreate1 {
    pub fn new(raw: RawSyscall) -> Self {
        let size = Integer::new(raw.args[0]);
        let retval = None;
        Self { size, retval }
    }
}
impl DecodeEntry for EpollCreate1 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.size.decode(pid, operation).unwrap();
    }
}

// int epoll_ctl(int epfd, int op, int fd, struct epoll_event *_Nullable event);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollCtl {
    pub epfd: Fd,
    pub op: Integer,
    pub fd: Fd,
    pub event: Struct,
    pub retval: Option<Integer>,
}
impl EpollCtl {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let op = Integer::new(raw.args[1]);
        let fd = Fd::new(raw.args[2]);
        let event = Struct::new(raw.args[3], Direction::InOut);
        let retval = None;
        Self { epfd, op, fd, event, retval }
    }
}
impl DecodeEntry for EpollCtl {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation).unwrap();
        self.op.decode(pid, operation).unwrap();
        self.fd.decode(pid, operation).unwrap();
        self.event.decode(pid, operation).unwrap();
    }
}

// int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollWait {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Integer,
    pub retval: Option<Integer>,
}
impl EpollWait {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Integer::new(raw.args[3]);
        let retval = None;
        Self { epfd, events, maxevents, timeout, retval }
    }
}
impl DecodeEntry for EpollWait {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation).unwrap();
        self.events.decode(pid, operation).unwrap();
        self.maxevents.decode(pid, operation).unwrap();
        self.timeout.decode(pid, operation).unwrap();
    }
}

// int epoll_pwait(int epfd, struct epoll_event *events, int maxevents, int timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollPwait {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Integer,
    pub sigmask: Struct,
    pub retval: Option<Integer>,
}
impl EpollPwait {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Integer::new(raw.args[3]);
        let sigmask = Struct::new(raw.args[4], Direction::In);
        let retval = None;
        Self { epfd, events, maxevents, timeout, sigmask, retval }
    }
}
impl DecodeEntry for EpollPwait {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation).unwrap();
        self.events.decode(pid, operation).unwrap();
        self.maxevents.decode(pid, operation).unwrap();
        self.timeout.decode(pid, operation).unwrap();
        self.sigmask.decode(pid, operation).unwrap();
    }
}

// int epoll_pwait2(int epfd, struct epoll_event *events, int maxevents, const struct timespec *_Nullable timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct EpollPwait2 {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Struct,
    pub sigmask: Struct,
    pub retval: Option<Integer>,
}
impl EpollPwait2 {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Struct::new(raw.args[3], Direction::In);
        let sigmask = Struct::new(raw.args[4], Direction::In);
        let retval = None;
        Self { epfd, events, maxevents, timeout, sigmask, retval }
    }
}
impl DecodeEntry for EpollPwait2 {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation).unwrap();
        self.events.decode(pid, operation).unwrap();
        self.maxevents.decode(pid, operation).unwrap();
        self.timeout.decode(pid, operation).unwrap();
        self.sigmask.decode(pid, operation).unwrap();
    }
}
