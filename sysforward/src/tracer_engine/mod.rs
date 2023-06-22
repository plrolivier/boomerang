/*
 * The tracer engine takes care of handling syscalls.
 */
pub mod decoder;
pub mod filtering;


use std::{
    sync::{ Arc },
    io::{ self },
};
use nix::{
    libc::user_regs_struct,
    unistd::{ Pid },
};
use serde_json;
use crate::{
    syscall::{ Syscall },
    arch::{ TargetArch, Architecture },
    operation::{ Operation, Ptrace },
    //protocol::{ Command, Packet, Header, SendSyscallEntryPayload, Client },
    tracer_engine::{
        decoder::{ Decoder },
        filtering::{ Decision, Filter },
    },
    protocol::data::Client,
};



/*
 * The tracer trait describes the interface a debugger should implement to be compatible with the control channel
 * and controlled by avatar2.
 */
pub trait TracerCallback {
    fn spawn_process(&mut self, program: &str, prog_args: &[&str]) -> Result<Pid, io::Error>;
    fn kill_process(&self, pid: Pid) -> Result<(), io::Error>;
    fn start_tracing(&self, pid: Pid) -> Result<(), io::Error>;
    fn stop_tracing(&self, pid: Pid) -> Result<(), io::Error>;
}


/*
 * The tracer engine implements the instrumentation for tracing a system call.
 */
pub struct TracerEngine {

    pub pid: i32,
    pub arch: Arc<Architecture>,
    //pub regs: Vec<u64>,
    pub regs: user_regs_struct,     // only for x86_64

    syscall: Syscall,
    //remote_syscall: Syscall,
    insyscall: bool,
    filter: Filter,

    interceptor: Box<dyn Operation>,
    decoder: Arc<Decoder>,
    protocol: Client,
}

impl TracerEngine {

    pub fn new(
        pid: i32,
        target_arch: TargetArch,
        ipv4_address: &str,
        tracer_port: u16,
        executor_port: u16,
    ) -> Self 
    {
        let arch = Arc::new(Architecture::new(target_arch));
        let decoder = Arc::new(Decoder::new(arch.clone()));

        Self {
            pid: pid,
            arch: arch,
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
            //remote_syscall: Syscall::new(),
            insyscall: false,   // Hypothesis: we do the tracing from the start!
            filter: Filter::new(String::from("filtername")),
            interceptor: Box::new(Ptrace {}),
            decoder: decoder,
            protocol: Client::new(ipv4_address, tracer_port, executor_port),
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

        // TODO: Add an option to decode only certain syscalls to increase speed.
        self.decoder.decode_entry(&mut self.syscall, self.pid, &self.interceptor);

        self.filter_entry();
        self.log_entry();

        self.carry_out_entry_decision();

        self.insyscall = true;
    }

    fn trace_exit(&mut self) {
        self.log_raw_exit();

        self.decoder.decode_exit();

        self.filter_exit();
        self.log_exit();

        self.carry_out_exit_decision();

        self.insyscall = false;
    }


    fn log_raw_entry(&self) {
        println!("[{}] [ENTRY] no: {:#x} args: {:x?}", 
                 self.pid, self.syscall.raw.no as usize, self.syscall.raw.args)
    }

    fn log_raw_exit(&self) {
        println!("[{}] [EXIT] retval: {:#x}", 
                 self.pid, self.syscall.raw.retval as usize)
    }

    fn log_entry(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[TRACER] {}", json)
    }

    fn log_exit(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[TRACER] {}", json);
        println!("");
    }

    fn filter_entry(&mut self) -> Option<Decision> {
        self.syscall.decision = Some(self.filter.filter(&self.syscall));
        self.syscall.decision
    }

    fn filter_exit(&self) -> Option<Decision> {
        self.syscall.decision
    }

    fn carry_out_entry_decision(&mut self) {
        match self.syscall.decision {
            Some(Decision::Continue) => (),
            _ => panic!("Decision not implemented")
        }

        // TODO: implement execute_decision()
        self.protocol.send_syscall_entry(&self.syscall);
    }

    fn carry_out_exit_decision(&mut self) {
        match self.syscall.decision {
            Some(Decision::Continue) => (),
            _ => panic!("Decision not implemented")
        }

    }


    /*
     * API to read/write from the environment by the interceptor "backend".
     */
    pub fn read_registers(&self) -> Option<user_regs_struct> {
        self.interceptor.read_registers(self.pid)
    }

    pub fn write_registers(&self, regs: user_regs_struct) -> bool {
        self.interceptor.write_registers(self.pid, regs)
    }

    pub fn read_memory(&self, addr: u64, size: u64) -> Vec<u8> {
        self.interceptor.read_memory(self.pid, addr, size)
    }

    pub fn write_memory(&self, addr: u64, mem: Vec<u8>) -> u64 {
        self.interceptor.write_memory(self.pid, addr, mem)
    }

    /*
    pub fn read_syscall_args(&self) -> Vec<u64> {
        self.interceptor.read_syscall_args(self.pid)
    }

    pub fn write_syscall_args(&self, args: Vec<u64>) -> bool {
        self.interceptor.write_syscall_args(self.pid, args)
    }

    pub fn read_syscall_ret(&self) -> (u64, u64) {
        self.interceptor.read_syscall_ret(self.pid)
    }

    pub fn write_syscall_ret(&self, retval: u64, errno: u64) -> bool {
        self.interceptor.write_syscall_ret(self.pid, retval, errno)
    }
    */
}
