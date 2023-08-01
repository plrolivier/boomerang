/*
 * The tracer engine implements the instrumentation for tracing a system call.
 */
/*
 * The tracer engine takes care of handling syscalls.
 */
use std::{
    collections::HashMap,
    sync::Arc,
    io,
};
use nix::libc::user_regs_struct;
use serde_json;
use crate::{
    arch::{ TargetArch, Architecture },
    protocol::data::Client,
    syscall::{
        Syscall,
        decoder::{ Decoder, DecodedSyscall },
    },
    tracer::{
        filtering::{ Decision, Filter, Rule },
        file_descriptor::FdTable,
    },
    targets::operation::Operation,
};


/*
struct TraceeState { 
    fd_table: HashMap<u16, FdLocation>,
}

impl TraceeState {
    pub fn new() -> Self {
        Self { fd_table: HashMap::new() }
    }
}
*/


pub struct TracerEngine {

    pub pid: i32,
    pub arch: Arc<Architecture>,
    //pub regs: Vec<u64>,
    pub regs: user_regs_struct,     // only for x86_64

    operator: Box<Operation>,
    decoder: Arc<Decoder>,
    protocol: Client,

    /* Tracee state */
    syscall: Syscall,
    remote_syscall: Syscall,
    insyscall: bool,
    fwd_fd_table: FdTable,
    //state: TraceeState,

    filter: Filter,

    saved_syscall: Vec<Syscall>,
}

impl TracerEngine {

    pub fn new(
        pid: i32,
        target_arch: TargetArch,
        ipv4_address: &str,
        tracer_port: u16,
        executor_port: u16,
        operator: Box<Operation>,
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
            operator: operator,
            decoder: decoder,
            protocol: Client::new(ipv4_address, tracer_port, executor_port),
            syscall: Syscall::new(),
            remote_syscall: Syscall::new(),
            insyscall: false,   // Hypothesis: we do the tracing from the start!
            fwd_fd_table: FdTable::new(),
            //state: TraceeState::new(),
            filter: Filter::new(String::from("filtername")),
            saved_syscall: Vec::new(),
        }
    }


    /*
     * When the tracking of the syscall entry/exit is left to the library,
     * we only synchronize the registers.
     */
    pub fn sync_registers(&mut self, regs: user_regs_struct) {
        self.regs = regs.clone();
    }

    pub fn trace(&mut self) -> Result<(), io::Error>
    {
        println!("fd_table: {:?}", self.fwd_fd_table);

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
        Ok(())
    }

    fn sync_entry(&mut self) {
        self.syscall = Syscall::new();
        self.remote_syscall = Syscall::new();

        // Only for x86_64
        self.set_syscall_entry(self.regs.orig_rax as usize,
                               self.regs.rdi as usize,
                               self.regs.rsi as usize,
                               self.regs.rdx as usize,
                               self.regs.r10 as usize,
                               self.regs.r8 as usize,
                               self.regs.r9 as usize,
                               0 as usize,
        );
    }

    fn sync_exit(&mut self) {
        // Only for x86_64
        self.set_syscall_exit(self.regs.rax as usize, self.regs.rdx as usize);
    }

    /*
     * The other way is to directly call the right method.
     */
    pub fn set_syscall_entry(&mut self, scno: usize, arg1: usize, 
                             arg2: usize, arg3: usize, arg4: usize,
                             arg5: usize, arg6: usize, arg7: usize) {
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

    pub fn set_syscall_exit(&mut self, retval: usize, errno: usize) {
        self.syscall.raw.retval = retval;
        self.syscall.raw.errno = errno;
    }

    pub fn shutdown(&mut self) -> Result<(), io::Error>
    {
        // Calculate & print syscall statistics
        // syscall number | how many? | is_decoded? | name
        match self.calculate_stats() {
            Ok(syscall_stats) => {
                self.print_stats(syscall_stats);
            },
            Err(err) => {
                println!("Oops, something happens when calculating syscall statistics: {}", err);
            }
        }
        Ok(())
    }

    /* Tracing */

    fn trace_entry(&mut self) {
        //self._log_raw_entry();

        // TODO: Add an option to decode only certain syscalls to increase speed.
        self.decoder.decode_entry(&mut self.syscall, self.pid, &self.operator);

        self.filter_entry();
        self.log_entry();

        // Note: When should the decoded syscall be sync with the RawSyscall/tracee?
        self.carry_out_entry_decision();

        self.insyscall = true;
    }

    fn trace_exit(&mut self) {
        //self._log_raw_exit();

        self.decoder.decode_exit(&mut self.syscall, self.pid, &self.operator);

        self.filter_exit();
        self.log_exit();

        self.carry_out_exit_decision();

        self.insyscall = false;
    }


    fn _log_raw_entry(&self) {
        println!("[{}] [ENTRY] no: {:#x} args: {:x?}", 
                 self.pid, self.syscall.raw.no as usize, self.syscall.raw.args)
    }

    fn _log_raw_exit(&self) {
        println!("[{}] [EXIT] retval: {:#x}", 
                 self.pid, self.syscall.raw.retval as usize)
    }

    fn log_entry(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] LOCAL: {}", self.pid, json)
    }

    fn log_exit(&mut self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] LOCAL: {}", self.pid, json);
        println!("");

        self.saved_syscall.push(self.syscall.clone());
    }

    /* Filtering */

    fn filter_entry(&mut self) -> Option<Decision> {
        self.syscall.decision = Some(self.filter.filter(self.insyscall, &self.syscall));
        self.syscall.decision
    }

    fn filter_exit(&mut self) -> Option<Decision> {
        self.syscall.decision = Some(self.filter.filter(self.insyscall, &self.syscall));
        self.syscall.decision
    }

    fn carry_out_entry_decision(&mut self)
    {
        //TODO: finish implementing the decisions
        match self.syscall.decision {
            Some(Decision::Continue) => {
                self.continue_entry().unwrap();
            },
            Some(Decision::Forward) => {
                self.forward_entry().unwrap();
            },
            _ => panic!("Decision not implemented")
        }
    }

    fn carry_out_exit_decision(&mut self)
    {
        // TODO: finish implementing the decisions
        // first the instrumentation, then the filter callback
        match self.syscall.decision {
            Some(Decision::Continue) => {
                self.continue_exit().unwrap();
                self.filter.on_syscall_exit(&self.syscall);
            },
            Some(Decision::Forward) => {
                self.forward_exit().unwrap();
                self.filter.on_syscall_exit(&self.remote_syscall);
            },
            _ => panic!("Decision not implemented")
        }

    }

    fn continue_entry(&mut self) -> Result<(), io::Error>
    {
        // TODO
        Ok(())
    }

    fn continue_exit(&mut self) -> Result<(), io::Error>
    {
        // TODO
        Ok(())
    }

    /* Forwarding */

    fn forward_entry(&mut self) -> Result<(), io::Error>
    {
        /* Pre-forward instrumentation */
        self.instr_pre_forward().unwrap();

        /* Forward */
        self.remote_syscall = self.protocol.send_syscall_entry(&self.remote_syscall).unwrap();
        //println!("[{}] remote syscall retval: {:#x}", self.pid, self.remote_syscall.raw.retval as usize);
        let json = serde_json::to_string(&self.remote_syscall).unwrap();
        println!("[{}] REMOTE: {}", self.pid, json);

        /* Post-forward instrumentation */
        self.instr_post_forward().unwrap();

        Ok(())
    }

    fn instr_pre_forward(&mut self) -> Result<(), io::Error>
    {
        /* Syscall specific instrumentation */
        self.remote_syscall = self.syscall.clone();
        match self.remote_syscall.name.as_str() {
            "close" => {
                // translate the fd with the remote fd
                if let DecodedSyscall::Close(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let user_fd = remote_syscall.fd.value;
                    let kernel_fd = self.fwd_fd_table.translate(user_fd).unwrap();
                    remote_syscall.fd.value = kernel_fd;
                }
            },
            "read" => {
                // translate the fd with the remote fd
                if let DecodedSyscall::Read(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let user_fd = remote_syscall.fd.value;
                    let kernel_fd = self.fwd_fd_table.translate(user_fd).unwrap(); // BUG=> la conversion ne s'est pas bien passe avec openat
                    remote_syscall.fd.value = kernel_fd;
                }
            },
            "write" => {
                // translate the fd with the remote fd
                if let DecodedSyscall::Write(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let user_fd = remote_syscall.fd.value;
                    let kernel_fd = self.fwd_fd_table.translate(user_fd).unwrap();
                    remote_syscall.fd.value = kernel_fd;
                }
            },
            "lseek" => {
                // translate the fd with the remote fd
                if let DecodedSyscall::Lseek(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let user_fd = remote_syscall.fd.value;
                    let kernel_fd = self.fwd_fd_table.translate(user_fd).unwrap(); // BUG=> la conversion ne s'est pas bien passe avec openat
                    remote_syscall.fd.value = kernel_fd;
                }
            }
            _ => (),
        };

        /* Replace local syscall with a dummy one */
        // note: it would be more clean to modify self.syscall.raw values and synchronized once we return to the program execution.
        // for now on x86-64, replace with getpid()
        let mut regs = self.operator.register.read_registers(self.pid).unwrap();
        regs.orig_rax = 39 as u64;  // getpid() in x86_64
        self.operator.register.write_registers(self.pid, regs).unwrap();

        Ok(())
    }

    fn instr_post_forward(&mut self) -> Result<(), io::Error>
    {
        match self.remote_syscall.name.as_str() {
            "open"  => {
                // a bit ugly but we replace the return value with the remote fd to not overlap with local fd space.
                if let DecodedSyscall::Open(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let retval = remote_syscall.retval.as_ref().unwrap().value;
                    if retval as i64 >= 0 {
                        let user_fd = self.fwd_fd_table.open_remote(retval);
                        remote_syscall.retval.as_mut().unwrap().value = user_fd;
                        self.remote_syscall.raw.retval = user_fd;
                    }
                }
            },
            "openat" => {
                // a bit ugly but we replace the return value with the remote fd to not overlap with local fd space.
                if let DecodedSyscall::Openat(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let retval = remote_syscall.retval.as_ref().unwrap().value;
                    if retval as i64 >= 0 {
                        let user_fd = self.fwd_fd_table.open_remote(retval);
                        remote_syscall.retval.as_mut().unwrap().value = user_fd;
                        self.remote_syscall.raw.retval = user_fd;
                    }
                }
            }
            "close" => {
                // on successful close, remove the fd from the table
                if let DecodedSyscall::Close(remote_syscall) = self.remote_syscall.decoded.as_mut().unwrap() {
                    let retval = remote_syscall.retval.as_ref().unwrap().value;
                    let user_fd = remote_syscall.fd.value;
                    if retval as i64 >= 0 {
                        let _kernel_fd = self.fwd_fd_table.close_remote(user_fd);
                    }
                }
            },
            _ => (),
        };
        Ok(())
    }

    fn forward_exit(&mut self) -> Result<(), io::Error>
    {
        /* 
         * On local syscall exit, this is usually the moment when the forwarded syscall synchronizes 
         * its state and its side-effects on the local system.
         */
        // TODO
        //self.write_syscall_ret(self.remote_syscall.raw.retval, self.remote_syscall.raw.errno)?;

        match self.remote_syscall.name.as_str() {
            "read" => {
                // sync the memory buffer
                // TODO: should be automated by passing over all arguments
                if let DecodedSyscall::Read(remote_syscall) = self.remote_syscall.decoded.as_ref().unwrap() {
                    let size = remote_syscall.buf.size;
                    let mem = &remote_syscall.buf.content[0..size];
                    let addr = remote_syscall.buf.address;
                    self.operator.memory.write(self.pid, addr, mem.to_vec());
                }
            }
            _ => (),
        };

        /* Syncrhonize back the return value and errno */
        let mut regs = self.operator.register.read_registers(self.pid).unwrap();
        regs.rax = self.remote_syscall.raw.retval as u64;
        regs.rdx = self.remote_syscall.raw.errno as u64;
        self.operator.register.write_registers(self.pid, regs).unwrap();

        // verify the register write...
        let regs = self.operator.register.read_registers(self.pid).unwrap();
        println!("fwd exit regs: {:?}", regs);

        Ok(())
    }

    /*
    fn update_fd_table(&mut self)
    {
        let sc: &dyn Any = self.syscall.decoded.as_ref().unwrap().get_syscall();
        match self.syscall.name.as_str() {
            "open" => {
            },
            "close" => {
            }
            _ => (),
        };
    }
    */

    /* Filtering management */

    pub fn load_rule(&mut self, index: usize, rule: Box<dyn Rule>)
    {
        self.filter.insert(index, rule)
    }

    pub fn unload_rule(&mut self, index: usize) -> Box<dyn Rule>
    {
        self.filter.remove(index)
    }

    /* Statistics */

    fn calculate_stats(&self) -> Result<HashMap<(usize, String), i32>, io::Error>
    {
        let mut syscall_stats: HashMap<(usize, String), i32> = HashMap::new();

        for syscall in &self.saved_syscall {
            let key = (syscall.raw.no.clone(), syscall.name.clone());
            let count = syscall_stats.entry(key).or_insert(0);
            *count += 1;
        }
        Ok(syscall_stats)
    }

    fn print_stats(&self, syscall_stats: HashMap<(usize, String), i32>)
    {
        //println!("{:?}", syscall_stats);

        println!("+-----+------------------+--------+");
        println!("| No  |       Name       | Number |");
        println!("+-----+------------------+--------+");

        for ((no, name), number) in syscall_stats {
            let name_str = if name.is_empty() { "<empty>" } else { &name };
            println!(
                "| {:<3} | {:<16} | {:<6} |",
                no, name_str, number
            );
        }
        println!("+-----+------------------+--------+");
    }



    /*
     * API to read/write from the environment by the interceptor "backend".
    pub fn read_registers(&self) -> Option<user_regs_struct> {
        self.operator.read_registers(self.pid)
    }

    //pub fn write_registers(&self, regs: user_regs_struct) -> Result<(), io::Error> {
    pub fn write_registers(&self, regs: user_regs_struct) -> bool {
        self.operator.write_registers(self.pid, regs)
    }

    pub fn read_memory(&self, addr: u64, size: u64) -> Vec<u8> {
        self.operator.read_memory(self.pid, addr, size)
    }

    pub fn write_memory(&self, addr: u64, mem: Vec<u8>) -> u64 {
        self.operator.write_memory(self.pid, addr, mem)
    }
     */

    /*
    pub fn read_syscall_args(&self) -> Vec<u64> {
        self.interceptor.read_syscall_args(self.pid)
    }

    pub fn write_syscall_args(&self, args: Vec<u64>) -> Result<(), io::Error> {
        self.interceptor.write_syscall_args(self.pid, args)
    }

    pub fn read_syscall_ret(&self) -> (u64, u64) {
        self.interceptor.read_syscall_ret(self.pid)
    }

    pub fn write_syscall_ret(&self, retval: u64, errno: u64) -> Result<(), io::Error> {
        self.interceptor.write_syscall_ret(self.pid, retval, errno)
    }
    */
}