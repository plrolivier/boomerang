/*
 *
 */
//pub mod invoker;


use crate::{
    arch::{ TargetArch, Architecture },
    syscall::{ Syscall },
    protocol::{ Server },
};



pub struct Executor {
    pub arch: Architecture,
    protocol: Server,

    syscall: Syscall,
}

impl Executor {

    pub fn new(target_arch: TargetArch) -> Self {
        Self {
            arch: Architecture::new(target_arch),
            syscall: Syscall::new(),
            protocol: Server::new(),
        }
    }

    pub fn run(&mut self) {

        /* The main loop */
        loop {
            /* Wait for new syscall */
            self.syscall = self.protocol.receive_syscall();

            /* Carry out syscall's decision */
            // TODO
            self.log_syscall();

            /* Return syscall */
            self.protocol.return_syscall_exit(&self.syscall);
        }
    }

    fn log_syscall(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[EXECUTOR] {}", json)
    }

}
