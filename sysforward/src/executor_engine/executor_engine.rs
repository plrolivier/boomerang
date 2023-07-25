/*
 *
 */
//pub mod invoker;

use std::{
    io::{ self }, sync::{Mutex, Condvar},
    sync::{ Arc },
};

use nix::{
    unistd::{ Pid },
};

use crate::{
    sync::{ Event },
    arch::{ TargetArch, Architecture },
    syscall::{ Syscall },
    operation::Operation,
    protocol::data::Server,
};




pub struct ExecutorEngine {
    pub arch: Architecture,
    protocol: Server,

    syscall: Syscall,
    operator: Box<Operation>,

    stop: Arc<Event>,
    stopped: Arc<Event>,
}

impl ExecutorEngine {

    pub fn new(
        target_arch: TargetArch,
        ipv4_address: &str,
        executor_port: u16,
        tracer_port: u16,
        stop_event: Arc<Event>,
        stopped_event: Arc<Event>,
        operator: Box<Operation>,
    ) -> Self
    {
        Self {
            arch: Architecture::new(target_arch),
            protocol: Server::new(ipv4_address, executor_port, tracer_port),
            syscall: Syscall::new(),
            operator: operator,
            stop: stop_event,
            stopped: stopped_event,
        }
    }

    pub fn run(&mut self)
    {
        self.init();

        /* The main loop */
        loop {
            if self.stop.is_set() {
                break;
            }

            /* Wait for new syscall
             * Note:
             * There is no timeout or keep-alive mechanisms to know when the tracer is finished.
             * Instead, the executor should run in another thread a listining loop to receive
             * remote commands (TODO).
             * For now, it needs to be stopped manually or via a signal.
             */
            //self.syscall = self.protocol.receive_syscall();
            match self.protocol.receive_syscall() {
                Ok(syscall) => {
                    self.syscall = syscall;
                },
                /* Unix => WouldBlock ; Windows => TimedOut
                Err(ref err) if err.kind() == io::ErrorKind::TimedOut => {
                    eprintln!("Socket timeout: {:?}", err);
                    continue;
                },
                */
                // The socket is set with a timeout of 1sec in order to check if the thread should stop.
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    //eprintln!("Socket timeout: {:?}", err);
                    continue;
                },
                Err(err) => {
                    eprintln!("An error occured: {:?}", err);
                    continue;
                }
            }

            /* Carry out syscall's decision */
            // TODO
            self.log_syscall();

            /* Return syscall */
            self.protocol.return_syscall_exit(&self.syscall);
        }

        self.stopped.set();

    }

    pub fn shutdown(&mut self)
    {
        if ! self.stopped.is_set() {
            self.stop.set();
            self.stopped.wait();
        }
        //Ok()
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