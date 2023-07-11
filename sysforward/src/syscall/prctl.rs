/*
 *
 */
use serde::{ Serialize, Deserialize };
use crate::{
    syscall::{ RawSyscall },
    syscall::args::{ Direction, Integer, Address },
    tracer::decoder::{ Decode },
    operation::{ Operation },
};


// int prctl(int option, unsigned long arg2, unsigned long arg3, unsigned long arg4, unsigned long arg5)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Prctl {
    pub option: Integer,
    pub arg2: Integer,
    pub arg3: Integer,
    pub arg4: Integer,
    pub arg5: Integer,
}
impl Prctl {
    pub fn new(raw: RawSyscall) -> Self {
        let option = Integer::new(raw.args[0]);
        let arg2 = Integer::new(raw.args[1]);
        let arg3 = Integer::new(raw.args[2]);
        let arg4 = Integer::new(raw.args[3]);
        let arg5 = Integer::new(raw.args[4]);
        Self { option, arg2, arg3, arg4, arg5 }
    }
}
impl Decode for Prctl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.option.decode(pid, operation);
        self.arg2.decode(pid, operation);
        self.arg3.decode(pid, operation);
        self.arg4.decode(pid, operation);
        self.arg5.decode(pid, operation);
    }
}


// int syscall(SYS_arch_prctl, int code, unsigned long addr)
// int syscall(SYS_arch_prctl, int code, unsigned long *addr)
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct ArchPrctl {
    pub code: Integer,
    pub addr: Address,
}
impl ArchPrctl {
    pub fn new(raw: RawSyscall) -> Self {
        let code = Integer::new(raw.args[0]);
        let addr = Address::new(raw.args[1], Direction::In);
        Self { code, addr }
    }
}
impl Decode for ArchPrctl {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        self.code.decode(pid, operation);
        self.addr.decode(pid, operation);
    }
}