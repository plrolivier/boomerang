/*
 *
 */
//pub mod invoker;

use std::{
    io::{ self },
};

use nix::{
    unistd::{ Pid },
};

use crate::{
    arch::{ TargetArch, Architecture },
    syscall::{ Syscall },
    operation::{ Operation, Ptrace },
    protocol::data::Server,
};



pub trait ExecutorCallback {
    fn spawn_process(&mut self, program: &str, prog_args: &[String]) -> Result<Pid, io::Error>;
    fn kill_process(&self, pid: Pid) -> Result<(), io::Error>;
}


pub struct ExecutorEngine {
    pub arch: Architecture,
    protocol: Server,

    syscall: Syscall,
    interceptor: Box<dyn Operation>,
}

impl ExecutorEngine {

    pub fn new(
        target_arch: TargetArch,
        ipv4_address: &str,
        executor_port: u16,
        tracer_port: u16,
    ) -> Self
    {
        Self {
            arch: Architecture::new(target_arch),
            protocol: Server::new(ipv4_address, executor_port, tracer_port),
            syscall: Syscall::new(),
            interceptor: Box::new(Ptrace {}),
        }
    }

    pub fn run(&mut self) {

        self.init();

        /* The main loop */
        loop {
            /* Wait for new syscall
             * Note:
             * There is no timeout or keep-alive mechanisms to know when the tracer is finished.
             * Instead, the executor should run in another thread a listining loop to receive
             * remote commands (TODO).
             * For now, it needs to be stopped manually or via a signal.
             */
            self.syscall = self.protocol.receive_syscall();

            /* Carry out syscall's decision */
            // TODO
            self.log_syscall();

            /* Return syscall */
            self.protocol.return_syscall_exit(&self.syscall);
        }
    }
    
    fn init(&mut self)
    {
        //self.protocol.init();
    }

    fn log_syscall(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[EXECUTOR] {}", json)
    }

}
