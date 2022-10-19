use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

//use libc::user_regs_struct;

use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};

mod tracer_engine;



fn main() {

    match unsafe{fork()} {

        Ok(ForkResult::Child) => {
            run_child();
        }

        Ok(ForkResult::Parent {child}) => {
            run_parent(child);
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    };
}


fn run_child() {
    ptrace::traceme().unwrap();
    Command::new("ls").exec();
    exit(0);
}

fn run_parent(child: Pid) {

    let mut tracer = tracer_engine::Tracer::new(child);

    wait().unwrap();    // exit syscall

    /*
     * The main loop of the program.
     */
    loop {
        wait_for_syscall(child);

        sync_registers(&mut tracer);

        tracer.trace();
    }
}



fn sync_registers(tracer: &mut tracer_engine::Tracer) {

    let regs = ptrace::getregs(tracer.pid).unwrap();

    tracer.regs.r15         = regs.r15;
    tracer.regs.r14         = regs.r14;
    tracer.regs.r13         = regs.r13;
    tracer.regs.r12         = regs.r12;
    tracer.regs.rbp         = regs.rbp;   
    tracer.regs.rbx         = regs.rbx;   
    tracer.regs.r11         = regs.r11;   
    tracer.regs.r10         = regs.r10;
    tracer.regs.r9          = regs.r9;
    tracer.regs.r8          = regs.r8;
    tracer.regs.rax         = regs.rax;
    tracer.regs.rcx         = regs.rcx;
    tracer.regs.rdx         = regs.rdx;
    tracer.regs.rsi         = regs.rsi;
    tracer.regs.rdi         = regs.rdi;
    tracer.regs.orig_rax    = regs.orig_rax;
    tracer.regs.rip         = regs.rip;
    tracer.regs.cs          = regs.cs;
    tracer.regs.eflags      = regs.eflags;
    tracer.regs.rsp         = regs.rsp;
    tracer.regs.ss          = regs.ss;
    tracer.regs.fs_base     = regs.fs_base;
    tracer.regs.gs_base     = regs.gs_base;
    tracer.regs.ds          = regs.ds;
    tracer.regs.es          = regs.es;
    tracer.regs.fs          = regs.fs;
    tracer.regs.gs          = regs.gs;

}


fn wait_for_syscall(child: Pid) {

    ptrace::syscall(child, None).unwrap();

    wait().unwrap();
    
    //TODO: handle return signal
}

