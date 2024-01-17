/*
 * A basic example to illustrate the use of the library for tracing system calls with ptrace.
 */
use std::{
    env,
    os::unix::process::CommandExt,
    process::{ self, Child, Command, Stdio },
    sync::Arc,
    io,
};

use nix::{
    sys::{
        ptrace,
        wait::{ waitpid, WaitStatus},
        signal::Signal,
    },
    unistd::Pid,
};

use sysfwd::{
    arch::{TargetArch, x86_64::ptrace::X86Ptrace },
    tracer::{ TracerEngine, filtering::Rule },
    targets::operation::Operation,
    memory::{ read_process_memory_maps, print_memory_regions },
};



/*
 * The debugger structure.
 */
struct TraceDebugger {
    target_arch: Arc<TargetArch>,
    program: String,
    prog_args: Vec<String>,
    tracee: Option<Child>,
}

impl TraceDebugger {

    pub fn new(target_arch: TargetArch, program: String, prog_args: Vec<String>) -> Self
    {
        Self {
            target_arch: Arc::new(target_arch),
            program: program,
            prog_args: prog_args,
            tracee: None,
        }
    }

    pub fn run(&mut self) -> Result<(),io::Error>
    {
        let tracer = self.boot_tracer(None).expect("Fail to setup the tracer");
        let _tracer = self.run_tracer(tracer).unwrap();
        Ok(())
    }


    fn boot_tracer(&mut self, rules: Option<Vec<Box<dyn Rule>>>) -> Result<TracerEngine, io::Error>
    {
        println!("Tracer PID {} booting...", process::id());

        /* Setup the tracee */
        self.spawn_tracee(self.program.clone(), self.prog_args.clone())?; 
        let pid = self.tracee.as_ref().unwrap().id() as i32;

        /* Show tracee initial memory map */
        let mem = read_process_memory_maps(pid as u32);
        print_memory_regions(&mem);

        /* Setup the tracer with the operators used to interact with memory, registers and syscalls */
        let x86ptrace_op = X86Ptrace::default();
        let regs_op = Box::new(x86ptrace_op.clone());
        let mem_op = Box::new(x86ptrace_op.clone());
        let sys_op = Box::new(x86ptrace_op);

        let operator = Box::new(Operation{ register: regs_op, memory: mem_op, syscall: sys_op });
        let arch = Arc::clone(&self.target_arch);
        //let arch_copy = self.target_arch.clone();

        let mut tracer = TracerEngine::new(pid,
                                           arch,
                                           None,
                                           None,
                                           None,
                                           operator,
                                          );

        /* Load filter rules */
        match rules {
            None => (),
            Some(rules) => {
                for rule in rules {
                    tracer.load_rule(0, rule);
                }
            }
        }

        Ok(tracer)
    }

    fn spawn_tracee(&mut self, program: String, prog_args: Vec<String>) -> Result<(), io::Error>
    {
        /*
        if self.use_pkexec {
            prog_args.insert(0, program);
            program = String::from_str("pkexec").unwrap();
        }
        */

        println!("Spawning: {} {:?}", program, prog_args);
        let mut command = Command::new(program);
        command.args(prog_args);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        unsafe {
            command.pre_exec(|| {
                ptrace::traceme().unwrap();
                // Disable ASLR for the program
                libc::personality(libc::ADDR_NO_RANDOMIZE.try_into().unwrap());
                Ok(())
            });
        }
        self.tracee = Some(command.spawn().expect("Failed to spawn child process"));

        /* 
        let status: std::process::ExitStatus = self.tracee.wait().unwrap();
        println!("Status is {:?}", status);
        println!("success: {:?}", status.success());
        println!("code: {:?}", status.code());
        */

        let pid = self.tracee.as_ref().unwrap().id();
        println!("Tracee PID {} spawn", pid);

        Ok(())
    }


    fn run_tracer(&mut self, mut tracer: TracerEngine) -> Result<TracerEngine, io::Error>
    {
        let pid = Pid::from_raw(self.tracee.as_ref().unwrap().id() as i32);

        /*
         * The main loop
         */
        loop {
            self.restart_syscall(pid).unwrap();

            match self.wait_for_syscall(pid) {
                Err(()) => break,

                Ok(_pid) => { 
                    tracer.trace()?;
                },
            }
        }
        Ok(tracer)
    }

    fn restart_syscall(&self, pid: Pid) -> Result<Pid, ()>
    {
        match ptrace::syscall(pid, None) {
            Ok(()) => { /* continue */ },
            Err(err) => {
                panic!("Fail to restart tracee: {:?}", err);
            }
            /*
            Err(ref err) if err.kind() == nix::errno::Errno::ESRCH => {
                println!("ESRCH: No such process: {:?}", err);
                return false;
            }
            */
        }
        Ok(pid)
    }

    fn wait_for_syscall(&self, pid: Pid) -> Result<Pid, ()>
    {
        match waitpid(pid, None) {
            Err(err) => {
                panic!("Oops something happens when waiting: {}", err);
            },

            Ok(status) => {
                match status {
                    WaitStatus::Stopped(pid, signo) => {
                        match signo {
                            Signal::SIGTRAP => {
                                return Ok(pid);
                            },
                            Signal::SIGSEGV => {
                                let regs = ptrace::getregs(pid).unwrap();
                                println!("Tracee {} segfault at {:#x}", pid, regs.rip);
                                return Err(());
                            },
                            // TODO: add support for other signals
                            _ => {
                                panic!("Tracee {} received signal {} which is not handled", pid, signo);
                            },
                        }
                    },
                    WaitStatus::Exited(pid, exit_status) => {
                        println!("The tracee {} exits with status {}", pid, exit_status);
                        return Err(());
                    },
                    // TODO: add support for other WaitStatus
                    _ => {
                        panic!("WaitStatus not handled");
                    },
                }
            },
        }
    }
}




fn main()
{
    // For now, the target architecture is the same for the debuggee program and the one where it runs.
    #[cfg(target_arch = "x86_64")]
    let arch = TargetArch::X86_64;
    #[cfg(target_arch = "mips")]
    let arch = TargetArch::Mipso32;


    /* TODO: add more argument to configure the tracer:
     *  - program to trace with its arguments
     *  - option on which port to listen
     *  - options for what to trace
     *  - architecture?
     *  - etc.
     * 
    */
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./ptracer <program> <arguments>");
        return;
    }

    let program = args[1].clone();
    let prog_args = args[2..].to_vec();


    let mut dbg = TraceDebugger::new(arch, program, prog_args);

    // Start tracing system calls
    println!("[TRACER] Start debugger...");
    dbg.run().unwrap();

    println!("[TRACER] Stop debugger");

}
