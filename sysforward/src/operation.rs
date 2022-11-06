/*
 * Operations is the interface used to retrieve information from the environment.
 * Typically, registers, memory, syscall arguments, etc.
 * They are implemented by the "backend" (ptrace, qemu-user, etc.) according to **how** syscall are intercepted.
 *
 * Note: we could at some point split the operations into different traits.
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



pub trait Operation {
    fn read_registers(&self, pid: Pid) -> Option<user_regs_struct>;
    fn write_registers(&self, pid: Pid, regs: user_regs_struct) -> bool;

    /* 
     * When it's possible to edit registers one by one:
    fn read_register(&self, pid: Pid, name: str) -> u64;
    fn write_register(&self, pid: Pid, name: str, value: u64) -> bool;
    */

    fn read_memory(&self, pid: Pid, addr: u64, size: u64) -> Vec<u32>;
    fn write_memory(&self, pid: Pid, addr: u64, mem: Vec<u32>) -> u64;

    /*
     * SyscallOperation allow to interact with the syscall values when it does not need to pass
     * by registers.
     * TODO It will be use and implemented later...
     *
    fn read_syscall_args(&self, pid: Pid) -> Vec<u64>;
    fn write_syscall_args(&self, pid: Pid, args: Vec<u64>) -> bool;
    fn read_syscall_ret(&self, pid: Pid) -> (u64, u64);
    fn write_syscall_ret(&self, pid: Pid, retval: u64, errno: u64) -> bool;
    */
}

/*
 * The first example of an interceptor uses ptrace to intercept syscalls.
 */
pub struct Ptrace { }

impl Operation for Ptrace {

    fn read_registers(&self, pid: Pid) -> Option<user_regs_struct> {
        Some(ptrace::getregs(pid).unwrap())
    }

    fn write_registers(&self, pid: Pid, regs: user_regs_struct) -> bool {
        match ptrace::setregs(pid, regs) {
            Result => return true,
            Error => return false,
        }
    }

    fn read_memory(&self, pid: Pid, addr: u64, size: u64) -> Vec<u32> {
        let mut mem = Vec::new();
        let mut addr = addr;
        let mut count = size + (4 - size % 4);

        while count > 0 {
            let address = addr as ptrace::AddressType;
            mem.push(ptrace::read(pid, address).unwrap() as u32);
            addr += 4;
            count -= 4;
        }
        mem
    }

    fn write_memory(&self, pid: Pid, addr: u64, mem: Vec<u32>) -> u64 {
        let mut addr = addr;
        let size = mem.len() as u64;
        let mut count = mem.len() as u64;
        let mut mem = VecDeque::from(mem);

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

/*
 * XXX 
 * Another way to had more flexibility between the interceptors would be to have a structure
 * which divide each trait Operations in subgroups (register, memory, syscall, etc.)
 *
struct Interceptor {
    register: Option<Box<dyn RegisterOperation>>,
    memory: Option<Box<dyn MemoryOperation>>,
    syscall: Option<Box<dyn SyscallOperation>>,
}

impl Interceptor {
    fn new(name: &str) -> Self {
        match name {
            "ptrace" => {
                let ptracer = Some(Box::new(Ptrace {}));
                return Self {
                    register: ptracer,
                    memory: ptracer,
                    syscall: None,
                }
            },
            _ => panic!("Interceptor {} not implemented", name),
        }
    }
}
*/
