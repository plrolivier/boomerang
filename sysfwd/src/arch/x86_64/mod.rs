/*
 *
 */
pub mod syscall_table;


use nix::libc::user_regs_struct;

use crate::{
    arch::UserRegister,
    syscall::Syscall,
};


 
#[derive(Clone, Debug)]
pub struct UserRegisterX86_64 {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rax: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    orig_rax: u64,
    rip: u64,
    cs: u64,
    eflags: u64,
    rsp: u64,
    ss: u64,
    fs_base: u64,
    gs_base: u64,
    ds: u64,
    es: u64,
    fs: u64,
    gs: u64,

}

impl UserRegisterX86_64 {
    pub fn new() -> Self {
        UserRegisterX86_64 {
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

    pub fn from(uregs: user_regs_struct) -> Self {
        UserRegisterX86_64 {
            r15: uregs.r15,
            r14: uregs.r14,
            r13: uregs.r13,
            r12: uregs.r12,
            rbp: uregs.rbp,
            rbx: uregs.rbx,
            r11: uregs.r11,
            r10: uregs.r10,
            r9: uregs.r9,
            r8: uregs.r8,
            rax: uregs.rax,
            rcx: uregs.rcx,
            rdx: uregs.rdx,
            rsi: uregs.rsi,
            rdi: uregs.rdi,
            orig_rax: uregs.orig_rax,
            rip: uregs.rip,
            cs: uregs.cs,
            eflags: uregs.eflags,
            rsp: uregs.rsp,
            ss: uregs.ss,
            fs_base: uregs.fs_base,
            gs_base: uregs.gs_base,
            ds: uregs.ds,
            es: uregs.es,
            fs: uregs.fs,
            gs: uregs.gs,
        }
    }

    pub fn set(&mut self, uregs: user_regs_struct) {
        self.r15 = uregs.r15;
        self.r14 = uregs.r14;
        self.r13 = uregs.r13;
        self.r12 = uregs.r12;
        self.rbp = uregs.rbp;
        self.rbx = uregs.rbx;
        self.r11 = uregs.r11;
        self.r10 = uregs.r10;
        self.r9 = uregs.r9;
        self.r8 = uregs.r8;
        self.rax = uregs.rax;
        self.rcx = uregs.rcx;
        self.rdx = uregs.rdx;
        self.rsi = uregs.rsi;
        self.rdi = uregs.rdi;
        self.orig_rax = uregs.orig_rax;
        self.rip = uregs.rip;
        self.cs = uregs.cs;
        self.eflags = uregs.eflags;
        self.rsp = uregs.rsp;
        self.ss = uregs.ss;
        self.fs_base = uregs.fs_base;
        self.gs_base = uregs.gs_base;
        self.ds = uregs.ds;
        self.es = uregs.es;
        self.fs = uregs.fs;
        self.gs = uregs.gs;
    }
}

impl UserRegister for UserRegisterX86_64 {

    fn set_syscall_entry(&self, syscall: &mut Syscall) {
        syscall.raw.no = self.orig_rax as usize;
        syscall.raw.args[0] = self.rdi as usize;
        syscall.raw.args[1] = self.rsi as usize;
        syscall.raw.args[2] = self.rdx as usize;
        syscall.raw.args[3] = self.r10 as usize;
        syscall.raw.args[4] = self.r8 as usize;
        syscall.raw.args[5] = self.r9 as usize;
        syscall.raw.args[6] = 0;
    }

    fn set_syscall_exit(&self, syscall: &mut Syscall) {
        syscall.raw.retval = self.rax as usize;
        syscall.raw.errno = self.rdx as usize;
    }
}
