pub mod tracer {

    use nix::unistd::Pid;


    pub struct X86Registers {
        pub r15: u64,
        pub r14: u64,
        pub r13: u64,
        pub r12: u64,
        pub rbp: u64,
        pub rbx: u64,
        pub r11: u64,
        pub r10: u64,
        pub r9: u64,
        pub r8: u64,
        pub rax: u64,
        pub rcx: u64,
        pub rdx: u64,
        pub rsi: u64,
        pub rdi: u64,
        pub orig_rax: u64,
        pub rip: u64,
        pub cs: u64,
        pub eflags: u64,
        pub rsp: u64,
        pub ss: u64,
        pub fs_base: u64,
        pub gs_base: u64,
        pub ds: u64,
        pub es: u64,
        pub fs: u64,
        pub gs: u64,
    }

    impl X86Registers {

        pub fn new() -> X86Registers {
            X86Registers {
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
            }
        }
    }


    struct Syscall {
        no: u64,
        args: [u64; 7],
        retval: u64,
        errno: u64,
        decision: Decision
    }

    impl Syscall {
        pub fn new() -> Syscall {
            Syscall {
                no: 0,
                args: [0, 0, 0, 0, 0, 0, 0,],
                retval: 0,
                errno: 0,
                decision: Decision::Continue,
            }
        }
    }


    #[derive(Clone, Copy)]
    enum Decision {
        Continue,
        FwdEntry,
        FwdExit,
        InspectExit,
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
        pub regs: X86Registers,

        syscall: Syscall,
        insyscall: bool,
        filter: Filter,
    }

    impl Tracer {

        pub fn new(pid: Pid) -> Tracer {
            Tracer {
                pid: pid,
                regs: X86Registers::new(),
                syscall: Syscall::new(),
                insyscall: true,
                filter: Filter::new(String::from("filtername")),
            }
        }

        pub fn trace(&mut self){
            match self.insyscall {
                true    => self.trace_entry(),
                false   => self.trace_exit(),
            }
            self.insyscall = !self.insyscall;
        }

        fn trace_entry(&mut self) {
            self.copy_entry();
            self.log_entry();

            match self.filter_entry() {
                Decision::Continue => (),
                _ => panic!("Decision is not Continue")
            }
        }

        fn trace_exit(&mut self) {
            self.copy_exit();
            self.log_exit();

            match self.filter_exit() {
                Decision::Continue => (),
                _ => panic!("Decision is not Continue")
            }
        }


        fn copy_entry(&mut self) {
            self.syscall.no = self.regs.orig_rax;
            self.syscall.args[0] = self.regs.rdi;
            self.syscall.args[1] = self.regs.rsi;
            self.syscall.args[2] = self.regs.rdx;
            self.syscall.args[3] = self.regs.r10;
            self.syscall.args[4] = self.regs.r8;
            self.syscall.args[5] = self.regs.r9;
            self.syscall.args[6] = 0;
        }

        fn copy_exit(&mut self) {
            self.syscall.retval = self.regs.orig_rax;
        }

        fn log_entry(&self) {
            println!("[ENTRY] no: {:#x} args: {:#x?}", 
                     self.syscall.no as usize, self.syscall.args)
        }

        fn log_exit(&self) {
            println!("[EXIT] retval: {:#x}", 
                     self.syscall.retval as usize)
        }

        fn filter_entry(&self) -> Decision {
            self.filter.filter(&self.syscall)
        }

        fn filter_exit(&self) -> Decision {
            self.syscall.decision
        }
    }

}
