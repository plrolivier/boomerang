/*
 * The first example of an interceptor uses ptrace to intercept syscalls.
 */
use core::ffi::c_void;
use std::{
    collections::VecDeque,
};
use nix::{
    unistd::Pid,
    libc::user_regs_struct,
    sys::ptrace,
};
use crate::{
    operation::Operation,
};



pub struct Ptrace { }

impl Operation for Ptrace {

    fn read_registers(&self, pid: i32) -> Option<user_regs_struct> {
        let pid = Pid::from_raw(pid);
        Some(ptrace::getregs(pid).unwrap())
    }

    fn write_registers(&self, pid: i32, regs: user_regs_struct) -> bool {
        let pid = Pid::from_raw(pid);
        match ptrace::setregs(pid, regs) {
            Ok(()) => return true,
            Err(e) => {
                eprintln!("Error setting registers for process {}: {}", pid, e);
                return false
            },
        }
    }

    fn read_memory(&self, pid: i32, addr: u64, size: u64) -> Vec<u8> {
        let pid = Pid::from_raw(pid);
        let mut mem: Vec<u8> = Vec::new();
        let mut addr = addr;
        let mut count = size + (4 - size % 4);

        while count > 0 {
            let address = addr as ptrace::AddressType;
            //mem.push(ptrace::read(pid, address).unwrap() as u32);
            let word = ptrace::read(pid, address).unwrap() as u32;
            mem.extend_from_slice(&word.to_le_bytes());
            addr += 4;
            count -= 4;
        }
        mem
    }

    fn write_memory(&self, pid: i32, addr: u64, mem: Vec<u8>) -> u64 {
        let pid = Pid::from_raw(pid);
        let mut addr = addr;
        let size = mem.len() as u64;
        let mut count = mem.len() as u64;
        let mut mem: VecDeque<u8> = VecDeque::from(mem);

        while count > 0 {
            let address = addr as ptrace::AddressType;
            let word = mem.pop_front().unwrap() as *mut c_void;
            unsafe {
                ptrace::write(pid, address, word).unwrap();
            }
            addr += 4;
            count -= 4;
        }
        size - count
    }

}
