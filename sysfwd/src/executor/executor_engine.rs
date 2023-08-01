/*
 *
 */
use std::{
    io::{ self },
    sync::Arc,
};

use crate::{
    sync::Event,
    arch::{ TargetArch, Architecture },
    protocol::data::Server,
    syscall::{
        Syscall,
        decoder::DecodeExit,
        encoder::EncodeEntry,
    },
    targets::operation::Operation,
    executor::Invoker,
};



pub struct ExecutorEngine {
    pub arch: Architecture,
    protocol: Server,

    syscall: Syscall,
    child_pid: i32,
    operator: Box<Operation>,
    invoker: Box<dyn Invoker>,

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
        invoker: Box<dyn Invoker>,
        child_pid: i32,
    ) -> Self
    {
        Self {
            arch: Architecture::new(target_arch),
            protocol: Server::new(ipv4_address, executor_port, tracer_port),
            syscall: Syscall::new(),
            operator: operator,
            stop: stop_event,
            stopped: stopped_event,
            invoker: invoker,
            child_pid: child_pid,
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
            self.log_entry_syscall();
            self.invoke_syscall().unwrap();
            self.log_exit_syscall();

            /* Return syscall */
            self.protocol.return_syscall_exit(&self.syscall);
        }

        self.stopped.set();

    }

    fn invoke_syscall(&mut self) -> Result<(), io::Error>
    {
        /* Encode the Syscall into a RawSyscall */

        // Let's take the hypothesis, the decoded syscall has not been modified,
        // and therefore does not need to be sync with the RawSyscall.
        //let raw = self.syscall.raw.clone();
        if let Some(decoded_sc) = self.syscall.decoded.as_mut() {
            let raw = self.syscall.raw.clone();
            self.syscall.raw = decoded_sc.encode_entry(raw, self.child_pid, &self.operator).unwrap();
        }

        /* Invoke the syscall */
        let (retval, errno)= self.invoker.invoke_syscall(self.syscall.raw.no,
                                 self.syscall.raw.args[0],
                                 self.syscall.raw.args[1],
                                 self.syscall.raw.args[2],
                                 self.syscall.raw.args[3],
                                 self.syscall.raw.args[4],
                                 self.syscall.raw.args[5],
                                 self.syscall.raw.args[6])
                                 .unwrap();
        self.syscall.raw.retval = retval;
        self.syscall.raw.errno = errno;

        /* Decode the syscall exit */
        if let Some(decoded_sc) = self.syscall.decoded.as_mut() {
            decoded_sc.decode_exit(self.syscall.raw.retval, self.child_pid, &self.operator).unwrap();
        }

        Ok(())
    }

    pub fn shutdown(&mut self)
    {
        if ! self.stopped.is_set() {
            self.stop.set();
            self.stopped.wait();
        }
    }
    
    fn init(&mut self)
    {
        //self.protocol.init();
    }


    fn log_entry_syscall(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] {}", self.child_pid, json);
    }

    fn log_exit_syscall(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[{}] {}", self.child_pid, json);
        println!("");
    }

}