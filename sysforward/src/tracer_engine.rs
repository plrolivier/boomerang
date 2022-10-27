/*
 *
 */
pub mod tracer {

    use nix::unistd::Pid;
    use nix::libc::user_regs_struct;

    use crate::arch::{ TargetArch, Architecture };


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
    }

    #[derive(Debug)]
    enum ArgType {
        /* Direct value */
        Integer(u64),
        Fd(u16),
        Size(u64),
        Offset(u64),
        Flag(u8),
        Prot(u8),
        Operation(u8),
        Signal(u8),
        /* Pointers */
        Address(u64),
        Buf(u64, Vec<u64>),
        NullBuf(u64, String),   // Null terminated buffer
        Struct(u64, String, Vec<u64>),
    }


    struct Syscall {
        raw: RawSyscall,
        name: String,
        args: Vec<Option<ArgType>>,
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
    }

    #[derive(Clone, Copy)]
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
        //pub regs: Vec<u64>,
        pub regs: user_regs_struct,     // only for x86_64

        syscall: Syscall,
        insyscall: bool,
        filter: Filter,

        arch: Architecture,
    }

    impl Tracer {

        pub fn new(pid: Pid, arch: TargetArch) -> Tracer {
            Tracer {
                pid: pid,
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
                arch: Architecture::new(arch)
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
            print!("[ENTRY] name: {} ", //args: {:#x?}", 
                     self.syscall.name);
            let mut args = Vec::new();
            for arg in &self.syscall.args {
                match arg {
                    Some(x) => args.push(x),
                    None => (),
                }
            }
            println!("args: {:?}", args);
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

            match self.syscall.name.as_str() {
                "open" => {
                    self.syscall.args[0] = Some(ArgType::NullBuf(self.syscall.raw.args[0], String::from("filename")));
                    self.syscall.args[1] = Some(ArgType::Flag(self.syscall.raw.args[1] as u8));
                    self.syscall.args[2] = Some(ArgType::Integer(self.syscall.raw.args[2]));
                },
                "openat" => {
                    self.syscall.args[0] = Some(ArgType::Integer(self.syscall.raw.args[0]));
                    self.syscall.args[1] = Some(ArgType::NullBuf(self.syscall.raw.args[1], String::from("filename")));
                    self.syscall.args[2] = Some(ArgType::Flag(self.syscall.raw.args[2] as u8));
                    self.syscall.args[3] = Some(ArgType::Integer(self.syscall.raw.args[3]));
                },
                "read" => {
                    self.syscall.args[0] = Some(ArgType::Fd(self.syscall.raw.args[0] as u16));
                    self.syscall.args[1] = Some(ArgType::Buf(self.syscall.raw.args[1], vec![0]));
                    self.syscall.args[2] = Some(ArgType::Size(self.syscall.raw.args[2]));
                },
                "write" => {
                    self.syscall.args[0] = Some(ArgType::Fd(self.syscall.raw.args[0] as u16));
                    self.syscall.args[1] = Some(ArgType::Buf(self.syscall.raw.args[1], vec![0]));
                    self.syscall.args[2] = Some(ArgType::Size(self.syscall.raw.args[2]));
                },
                "close" => {
                    self.syscall.args[0] = Some(ArgType::Fd(self.syscall.raw.args[0] as u16));
                },
                "mmap" => {
                    self.syscall.args[0] = Some(ArgType::Address(self.syscall.raw.args[0]));
                    self.syscall.args[1] = Some(ArgType::Size(self.syscall.raw.args[1]));
                    self.syscall.args[2] = Some(ArgType::Prot(self.syscall.raw.args[2] as u8));
                    self.syscall.args[3] = Some(ArgType::Fd(self.syscall.raw.args[3] as u16));
                    self.syscall.args[4] = Some(ArgType::Offset(self.syscall.raw.args[4]));
                },
                _ => (),
            }
        }
    }

}
