/*
 *
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Fd, Struct },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};



// int epoll_create(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCreate {
    pub size: Integer,
}
impl EpollCreate {
    pub fn new(raw: RawSyscall) -> Self {
        let size = Integer::new(raw.args[0]);
        Self { size }
    }
}
impl Decode for EpollCreate {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.size.decode(pid, operation);
    }
}

// int epoll_create1(int size);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCreate1 {
    pub size: Integer,
}
impl EpollCreate1 {
    pub fn new(raw: RawSyscall) -> Self {
        let size = Integer::new(raw.args[0]);
        Self { size }
    }
}
impl Decode for EpollCreate1 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.size.decode(pid, operation);
    }
}

// int epoll_ctl(int epfd, int op, int fd, struct epoll_event *_Nullable event);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollCtl {
    pub epfd: Fd,
    pub op: Integer,
    pub fd: Fd,
    pub event: Struct,
}
impl EpollCtl {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let op = Integer::new(raw.args[1]);
        let fd = Fd::new(raw.args[2]);
        let event = Struct::new(raw.args[3], Direction::InOut);
        Self { epfd, op, fd, event }
    }
}
impl Decode for EpollCtl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation);
        self.op.decode(pid, operation);
        self.fd.decode(pid, operation);
        self.event.decode(pid, operation);
    }
}

// int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollWait {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Integer,
}
impl EpollWait {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Integer::new(raw.args[3]);
        Self { epfd, events, maxevents, timeout }
    }
}
impl Decode for EpollWait {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation);
        self.events.decode(pid, operation);
        self.maxevents.decode(pid, operation);
        self.timeout.decode(pid, operation);
    }
}

// int epoll_pwait(int epfd, struct epoll_event *events, int maxevents, int timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollPwait {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Integer,
    pub sigmask: Struct,
}
impl EpollPwait {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Integer::new(raw.args[3]);
        let sigmask = Struct::new(raw.args[4], Direction::In);
        Self { epfd, events, maxevents, timeout, sigmask }
    }
}
impl Decode for EpollPwait {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation);
        self.events.decode(pid, operation);
        self.maxevents.decode(pid, operation);
        self.timeout.decode(pid, operation);
        self.sigmask.decode(pid, operation);
    }
}

// int epoll_pwait2(int epfd, struct epoll_event *events, int maxevents, const struct timespec *_Nullable timeout, const sigset_t *_Nullable sigmask);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EpollPwait2 {
    pub epfd: Fd,
    pub events: Struct,
    pub maxevents: Integer,
    pub timeout: Struct,
    pub sigmask: Struct,
}
impl EpollPwait2 {
    pub fn new(raw: RawSyscall) -> Self {
        let epfd = Fd::new(raw.args[0]);
        let events = Struct::new(raw.args[1], Direction::InOut);
        let maxevents = Integer::new(raw.args[2]);
        let timeout = Struct::new(raw.args[3], Direction::In);
        let sigmask = Struct::new(raw.args[4], Direction::In);
        Self { epfd, events, maxevents, timeout, sigmask }
    }
}
impl Decode for EpollPwait2 {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.epfd.decode(pid, operation);
        self.events.decode(pid, operation);
        self.maxevents.decode(pid, operation);
        self.timeout.decode(pid, operation);
        self.sigmask.decode(pid, operation);
    }
}
