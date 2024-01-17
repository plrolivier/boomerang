/*
 * Operations is the interface used to retrieve information from the environment.
 * Typically, registers, memory, syscall arguments, etc.
 * They are implemented by the "backend" (ptrace, qemu-user, etc.) according to **how** syscall are intercepted.
 *
 * Note: we could at some point split the operations into different traits.
 */

 // XXX: pid is i32 or libc::pid_t?
use std::rc::Rc;
use nix::sys;

use crate::arch::x86_64::ptrace::X86Ptrace;


pub trait RegisterOperation {
    fn read_registers(&mut self, pid: i32) -> Result<Vec<usize>, std::io::Error>;
    fn write_registers(&mut self, pid: i32, regs: Vec<usize>) -> Result<(), std::io::Error>;

    // TODO: Possible other methods to add
    //fn convert_regs_to_syscall(&self) -> Result<Syscall, std::io::Error>;
    //fn read_and_convert_syscall(&self, pid: i32) -> Result<Syscall, std::io::Error>;

    /* 
     * When it's possible to edit registers one by one:
    fn read_register(&self, pid: i32, name: str) -> u64;
    fn write_register(&self, pid: i32, name: str, value: u64) -> bool;
    */
}

pub trait MemoryOperation {
    fn read(&self, pid: i32, addr: usize, size: usize) -> Vec<u8>;
    fn write(&self, pid: i32, addr: usize, mem: Vec<u8>) -> usize;
}

/*
 * SyscallOperation allow to interact with the syscall values when it does not need to pass
 * by registers.
 * New: it can be used instead of register and does the conversion itself.
 */
pub trait SyscallOperation {
    fn read_syscall_args(&mut self, pid: i32) -> Result<Vec<usize>, std::io::Error>;
    fn write_syscall_args(&mut self, pid: i32, args: Vec<usize>) -> Result<(), std::io::Error>;

    fn read_syscall_ret(&mut self, pid: i32) -> Result<(usize, usize), std::io::Error>;
    fn write_syscall_ret(&mut self, pid: i32, retval: usize, errno: usize) -> Result<(), std::io::Error>;

    fn replace_syscall_no(&mut self, pid: i32, no: usize) -> Result<(), std::io::Error>;
}

/*
 * This API could be enhanced by offering the possibility of sharing backends
 * between different operations. This would reduce memory requirements and
 * remove the need to clone the same backend several times when you want to
 * associate it with several operations.
 */
pub struct Operation {
    pub register: Box<dyn RegisterOperation>,
    pub memory: Box<dyn MemoryOperation>,
    pub syscall: Box<dyn SyscallOperation>,
}




/*
 * XXX 
 * Another way to had more flexibility between the interceptors would be to have a structure
 * which divide each trait Operations in subgroups (register, memory, syscall, etc.).
 * In a similary way that avatar2 does.
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
