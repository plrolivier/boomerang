/*
 *
 */
pub mod tracer {

    use nix::unistd::Pid;
    use nix::libc::user_regs_struct;

    use crate::arch::{ TargetArch, Architecture };



    struct Syscall {
        no: u64,
        args: Vec<u64>,
        retval: u64,
        errno: u64,
        decision: Decision
    }

    impl Syscall {
        fn new() -> Syscall {
            Syscall {
                no: 0,
                args: vec![0; 7],
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
                insyscall: true,
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
                true    => {
                    self.sync_entry();
                    self.trace_entry();
                },

                false   => {
                    self.sync_exit();
                    self.trace_exit();
                },
            }
        }

        fn sync_entry(&mut self) {
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
            self.syscall.no = scno;
            self.syscall.args[0] = arg1;
            self.syscall.args[1] = arg2;
            self.syscall.args[2] = arg3;
            self.syscall.args[3] = arg4;
            self.syscall.args[4] = arg5;
            self.syscall.args[5] = arg6;
            self.syscall.args[6] = arg7;
        }

        pub fn set_syscall_exit(&mut self, retval: u64, errno: u64) {
            self.syscall.retval = retval;
            self.syscall.errno = errno;
        }

        fn trace_entry(&mut self) {
            self.log_entry();

            match self.filter_entry() {
                Decision::Continue => (),
                _ => panic!("Decision not implemented")
            }

            self.insyscall = true;
        }

        fn trace_exit(&mut self) {
            self.log_exit();

            match self.filter_exit() {
                Decision::Continue => (),
                _ => panic!("Decision not implemented")
            }

            self.insyscall = false;
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
