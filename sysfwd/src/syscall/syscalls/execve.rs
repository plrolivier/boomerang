/*
 *
 */
use serde::{ Serialize, Deserialize };
use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Fd, Flag, Address, NullBuffer },
    syscall::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    targets::operation::Operation,
};



// int execve(const char *pathname, char *const _Nullable argv[], char *const _Nullable envp[])
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Execve {
    pub pathname: NullBuffer,
    pub argv: Address,
    pub envp: Address,
    pub retval: Option<Integer>,
}
impl Execve {
    pub fn new(raw: RawSyscall) -> Self {
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let argv = Address::new(raw.args[1], Direction::In);
        let envp = Address::new(raw.args[2], Direction::In);
        let retval = None;
        Self { pathname, argv, envp, retval }
    }
}
impl DecodeEntry for Execve {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pathname.decode(pid, operation).unwrap();
        self.argv.decode(pid, operation).unwrap();
        self.envp.decode(pid, operation).unwrap();
    }
}

// int execveat(int dirfd, const char *pathname, char *const _Nullable argv[], char *const _Nullable envp[], int flags)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct Execveat {
    pub dirfd: Fd,
    pub pathname: NullBuffer,
    pub argv: Address,
    pub envp: Address,
    pub flags: Flag,
    pub retval: Option<Integer>,
}
impl Execveat {
    pub fn new(raw: RawSyscall) -> Self {
        let dirfd = Fd::new(raw.args[0]);
        let pathname = NullBuffer::new(raw.args[0], Direction::In);
        let argv = Address::new(raw.args[1], Direction::In);
        let envp = Address::new(raw.args[2], Direction::In);
        let flags = Flag::new(raw.args[4]);
        let retval = None;
        Self { dirfd, pathname, argv, envp, flags, retval }
    }
}
impl DecodeEntry for Execveat {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.dirfd.decode(pid, operation).unwrap();
        self.pathname.decode(pid, operation).unwrap();
        self.argv.decode(pid, operation).unwrap();
        self.envp.decode(pid, operation).unwrap();
        self.flags.decode(pid, operation).unwrap();
    }
}