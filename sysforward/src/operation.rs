/*
 * Operations is the interface used to retrieve information from the environment.
 * Typically, registers, memory, syscall arguments, etc.
 * They are implemented by the "backend" (ptrace, qemu-user, etc.) according to **how** syscall are intercepted.
 *
 * Note: we could at some point split the operations into different traits.
 */
use nix::{
    libc::user_regs_struct,
};



pub trait Operation {
    fn read_registers(&self, pid: i32) -> Option<user_regs_struct>;
    fn write_registers(&self, pid: i32, regs: user_regs_struct) -> bool;

    /* 
     * When it's possible to edit registers one by one:
    fn read_register(&self, pid: i32, name: str) -> u64;
    fn write_register(&self, pid: i32, name: str, value: u64) -> bool;
    */

    fn read_memory(&self, pid: i32, addr: u64, size: u64) -> Vec<u8>;
    fn write_memory(&self, pid: i32, addr: u64, mem: Vec<u8>) -> u64;

    /*
     * SyscallOperation allow to interact with the syscall values when it does not need to pass
     * by registers.
     * TODO It will be use and implemented later...
     *
    fn read_syscall_args(&self, pid: i32) -> Vec<u64>;
    fn write_syscall_args(&self, pid: i32, args: Vec<u64>) -> bool;
    fn read_syscall_ret(&self, pid: i32) -> (u64, u64);
    fn write_syscall_ret(&self, pid: i32, retval: u64, errno: u64) -> bool;
    */
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
