use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};

use sysforward::tracer_engine::tracer::Tracer;
use sysforward::arch::TargetArch;



fn main() {

    match unsafe {fork()} {

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

    let mut tracer = Tracer::new(child, TargetArch::X86_64);

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



fn sync_registers(tracer: &mut Tracer) {

    let regs = ptrace::getregs(tracer.pid).unwrap();
    tracer.sync_registers(regs);
}

fn wait_for_syscall(child: Pid) {

    ptrace::syscall(child, None).unwrap();

    wait().unwrap();
    
    //TODO: handle return signal
}

