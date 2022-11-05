/*
 * The tracer engine takes care of handling syscalls.
 */
pub mod tracer {

    use core::ffi::c_void;
    use std::collections::VecDeque;
    use std::fmt;

    use nix::unistd::Pid;
    use nix::libc::user_regs_struct;
    use nix::sys::ptrace;

    use crate::arch::{ TargetArch, Architecture };
    use crate::syscall::ArgumentType::{ Decode, Int, Fd, Size, Offset, Flag, Prot, Signal, Address, Buffer, NullBuf, Struct };


    struct RawSyscall {
        no: u64,
        args: Vec<u64>,
        retval: u64,
        errno: u64,
    }

    impl RawSyscall {
        fn new() -> Self {
            Self {
                no: 0,
                args: vec![0; 7],
                retval: 0,
                errno: 0,
            }
        }

        fn to_json(&self) -> String {
            format!("{{\"no\":{},\"args\":{:?},\"retval\":{},\"errno\":{}}}", self.no, self.args, self.retval, self.errno)
        }
    }


    struct Syscall {
        raw: RawSyscall,
        name: String,
        args: Vec<Option<Box<dyn Decode>>>,
        decision: Option<Decision>,
    }

    impl Syscall {
        fn new() -> Self {
            Self {
                raw: RawSyscall::new(),
                name: String::with_capacity(25),
                //args: vec![&None; 7],
                args: Vec::from([None, None, None, None, None, None, None]),
                //decision: None,
                decision: Some(Decision::Continue), //Once the filtering implemented, put None 
            }
        }

        fn decode(&mut self, pid: Pid, operation: &Box<dyn Operation>) {
            for arg in self.args.iter_mut() {
                match arg {
                    Some(a) => a.decode(pid, operation),
                    None => break,
                }
            }
        }

        fn print(&self) {
            for arg in self.args.iter() {
                match arg {
                    Some(a) => a.print(),
                    None => break,
                }
            }
        }

        fn args_to_json(&self) -> String {
            // TODO: improve format here
            let mut s = String::new();
            s.push('[');
            for arg in self.args.iter() {
                match arg {
                    Some(a) => s.push_str(&a.to_json()),
                    None => break,
                }
                s.push(',');    //TODO: always add a trailing comma...
            }
            s.push(']');
            s
        }

        fn to_json(&self) -> String {
            // TODO: replace 0 with self.decision
            format!("{{\"raw\":{},\"name\":\"{}\",\"args\":{},\"decision\":{}}}", self.raw.to_json(), self.name, self.args_to_json(), 0)
        }

    }

    #[derive(Clone, Copy, Debug)]
    enum Decision {
        Continue,
        FwdEntry,
        FwdExit,
        InspectExit,
        LogLocal,
        NoExec,
        Kill,
    }

    struct Filter {
        pub name: String,
        pub decision: Decision,
    }

    impl Filter {
        pub fn new(name: String) -> Filter {
            Filter {
                name: name,
                decision: Decision::Continue,
            }
        }

        pub fn filter(&self, _syscall: &Syscall) -> Decision {
            self.decision
        }
    }



    pub struct Tracer {

        pub pid: Pid,
        pub arch: Architecture,
        //pub regs: Vec<u64>,
        pub regs: user_regs_struct,     // only for x86_64

        syscall: Syscall,
        insyscall: bool,
        filter: Filter,

        interceptor: Box<dyn Operation>,
    }

    impl Tracer {

        pub fn new(pid: Pid, arch: TargetArch) -> Tracer {
            Tracer {
                pid: pid,
                arch: Architecture::new(arch),
                //regs: vec![0; 33],
                regs: user_regs_struct {
                    r15: 0,
                    r14: 0,
                    r13: 0,
                    r12: 0,
                    rbp: 0,
                    rbx: 0,
                    r11: 0,
                    r10: 0,
                    r9: 0,
                    r8: 0,
                    rax: 0,
                    rcx: 0,
                    rdx: 0,
                    rsi: 0,
                    rdi: 0,
                    orig_rax: 0,
                    rip: 0,
                    cs: 0,
                    eflags: 0,
                    rsp: 0,
                    ss: 0,
                    fs_base: 0,
                    gs_base: 0,
                    ds: 0,
                    es: 0,
                    fs: 0,
                    gs: 0,
                },
                syscall: Syscall::new(),
                insyscall: false,   // Hypothesis: we do the tracing from the start!
                filter: Filter::new(String::from("filtername")),
                interceptor: Box::new(Ptracer {}),
            }
        }


        /*
         * When the tracking of the syscall entry/exit is left to the library,
         * we only synchronize the registers.
         */
        pub fn sync_registers(&mut self, regs: user_regs_struct) {
            self.regs = regs.clone();
        }

        pub fn trace(&mut self){
            match self.insyscall {
                false    => {
                    self.sync_entry();
                    self.trace_entry();
                },

                true   => {
                    self.sync_exit();
                    self.trace_exit();
                },
            }
        }

        fn sync_entry(&mut self) {
            self.syscall = Syscall::new();

            // Only for x86_64
            self.set_syscall_entry(self.regs.orig_rax,
                                   self.regs.rdi,
                                   self.regs.rsi,
                                   self.regs.rdx,
                                   self.regs.r10,
                                   self.regs.r8,
                                   self.regs.r9,
                                   0,
            );
        }

        fn sync_exit(&mut self) {
            // Only for x86_64
            self.set_syscall_exit(self.regs.orig_rax, self.regs.rdx);
        }

        /*
         * The other way is to directly call the right method.
         */
        pub fn set_syscall_entry(&mut self, scno: u64, arg1: u64, 
                                 arg2: u64, arg3: u64, arg4: u64,
                                 arg5: u64, arg6: u64, arg7: u64) {
            // TODO: what about seccomp (see strace & PTRACE_GET_SYSCALL_INFO)
            self.syscall.raw.no = scno;
            self.syscall.raw.args[0] = arg1;
            self.syscall.raw.args[1] = arg2;
            self.syscall.raw.args[2] = arg3;
            self.syscall.raw.args[3] = arg4;
            self.syscall.raw.args[4] = arg5;
            self.syscall.raw.args[5] = arg6;
            self.syscall.raw.args[6] = arg7;
        }

        pub fn set_syscall_exit(&mut self, retval: u64, errno: u64) {
            self.syscall.raw.retval = retval;
            self.syscall.raw.errno = errno;
        }

        fn trace_entry(&mut self) {
            self.log_raw_entry();
            self.decode_entry();
            self.log_entry();

            match self.filter_entry() {
                Some(Decision::Continue) => (),
                _ => panic!("Decision not implemented")
            }

            self.insyscall = true;
        }

        fn trace_exit(&mut self) {
            self.log_raw_exit();
            self.log_exit();

            match self.filter_exit() {
                Some(Decision::Continue) => (),
                _ => panic!("Decision not implemented")
            }

            self.insyscall = false;
        }


        fn log_raw_entry(&self) {
            println!("[ENTRY] no: {:#x} args: {:x?}", 
                     self.syscall.raw.no as usize, self.syscall.raw.args)
        }

        fn log_raw_exit(&self) {
            println!("[EXIT] retval: {:#x}", 
                     self.syscall.raw.retval as usize)
        }

        fn log_entry(&self) {
            //print!("[ENTRY] name: {} ", //args: {:#x?}", 
            println!("[ENTRY] name: {} ", //args: {:#x?}", 
                     self.syscall.name);
            let mut args = Vec::new();
            for arg in &self.syscall.args {
                match arg {
                    Some(x) => args.push(x),
                    None => (),
                }
            }

            //self.syscall.print();
            println!("{}", self.syscall.to_json());
        }

        fn log_exit(&self) {
            println!("[EXIT] name: {}", self.syscall.name);
        }

        fn filter_entry(&mut self) -> Option<Decision> {
            self.syscall.decision = Some(self.filter.filter(&self.syscall));
            self.syscall.decision
        }

        fn filter_exit(&self) -> Option<Decision> {
            self.syscall.decision
        }

        fn decode_entry(&mut self) {
            // TODO: improve the match by using number instead of strings
            match self.arch.syscall_table.get_syscall_name(&self.syscall.raw.no) {
                Some(x) => self.syscall.name = x,
                None => println!("No name found for {}", self.syscall.raw.no),
            }

            /*
             * First, assign a type to each argument according to the syscall.
             */
            match self.syscall.name.as_str() {
                "open" => {
                    self.syscall.args[0] = Some(Box::new(NullBuf::new(self.syscall.raw.args[0])));
                    self.syscall.args[1] = Some(Box::new(Flag::new(self.syscall.raw.args[1])));
                    self.syscall.args[2] = Some(Box::new(Int::new(self.syscall.raw.args[2])));
                },
                "openat" => {
                    self.syscall.args[0] = Some(Box::new(Int::new(self.syscall.raw.args[0])));
                    self.syscall.args[1] = Some(Box::new(NullBuf::new(self.syscall.raw.args[1])));
                    self.syscall.args[2] = Some(Box::new(Flag::new(self.syscall.raw.args[2])));
                    self.syscall.args[3] = Some(Box::new(Int::new(self.syscall.raw.args[3])));
                },
                "read" => {
                    self.syscall.args[0] = Some(Box::new(Fd::new(self.syscall.raw.args[0])));
                    self.syscall.args[1] = Some(Box::new(Buffer::new(self.syscall.raw.args[1], self.syscall.raw.args[2])));
                    self.syscall.args[2] = Some(Box::new(Size::new(self.syscall.raw.args[2])));
                },
                "write" => {
                    self.syscall.args[0] = Some(Box::new(Fd::new(self.syscall.raw.args[0])));
                    self.syscall.args[1] = Some(Box::new(Buffer::new(self.syscall.raw.args[1], self.syscall.raw.args[2])));
                    self.syscall.args[2] = Some(Box::new(Size::new(self.syscall.raw.args[2])));
                },
                "close" => {
                    self.syscall.args[0] = Some(Box::new(Fd::new(self.syscall.raw.args[0])));
                },
                "mmap" => {
                    self.syscall.args[0] = Some(Box::new(Address::new(self.syscall.raw.args[0])));
                    self.syscall.args[1] = Some(Box::new(Size::new(self.syscall.raw.args[1])));
                    self.syscall.args[2] = Some(Box::new(Prot::new(self.syscall.raw.args[2])));
                    self.syscall.args[3] = Some(Box::new(Fd::new(self.syscall.raw.args[3])));
                    self.syscall.args[4] = Some(Box::new(Offset::new(self.syscall.raw.args[4])));
                },
                _ => (),
            }

            /*
             * Second, iterate over the argument to decode them.
             */
            self.syscall.decode(self.pid, &self.interceptor);
        }

        pub fn read_registers(&self) -> Option<user_regs_struct> {
            self.interceptor.read_registers(self.pid)
        }

        pub fn write_registers(&self, regs: user_regs_struct) -> bool {
            self.interceptor.write_registers(self.pid, regs)
        }

        pub fn read_memory(&self, addr: u64, size: u64) -> Vec<u32> {
            self.interceptor.read_memory(self.pid, addr, size)
        }

        pub fn write_memory(&self, addr: u64, mem: Vec<u32>) -> u64 {
            self.interceptor.write_memory(self.pid, addr, mem)
        }
    }

    /*
     * TODO
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
                    let ptracer = Some(Box::new(Ptracer {}));
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

    /*
     * Operations are implemented by the "backend" according to **how** syscall are intercepted.
     */
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
    struct Ptracer { }

    impl Operation for Ptracer {

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

}
