/*
 *
 */
pub mod invoker;


use crate::{
    arch::{ TargetArch, Architecture },
    syscall::{ Syscall },
    protocol::{
        worker::{ Worker },
        message::{ Message, Header, Command },
    },
};



pub struct Executor {
    pub arch: Architecture,
    protocol: Worker,

    syscall: Syscall,
}

impl Executor {

    pub fn new(target_arch: TargetArch) -> Self {
        Self {
            arch: Architecture::new(target_arch),
            syscall: Syscall::new(),
            protocol: Worker::new(),
        }
    }

    pub fn run(&mut self) {

        /* The main loop */
        loop {
            //let message = self.protocol.receive_message().expect("Received corrupted message");
            let message: Message = self.protocol.receive_message();

            self.dispatch_command(message);

            // TODO: execute syscall

            //self.reply_syscall_exit();
        }
    }

    fn dispatch_command(&mut self, message: Message)
    {
        let header = message.header;
        let payload = message.payload;

        match header.command {
            Command::SendSyscallEntry => {
                self.on_send_syscall_entry(payload.payload.unwrap());
            },
            Command::NotifyNewProcess => {
                self.on_notify_new_process(payload.payload.unwrap());
            },
            _ => {
                println!("[EXECUTOR] Command {:?} not implemented", header.command);
            }
        }

    }

    fn on_send_syscall_entry(&mut self, payload: Vec<u8>)
    {
        self.syscall = serde_json::from_slice(&payload).expect("Fail to deserialize syscall entry");
        
        // For now, only log the syscall
        self.log_syscall();

        self.protocol.return_syscall_exit(&self.syscall);
    }

    fn on_notify_new_process(&self, payload: Vec<u8>)
    {
        // TODO
        println!("TODO: notify_new_process");
    }

    /* Methods calls by the worker -> replace by invoker

    fn exec_syscall_entry(&self, syscall: Syscall)
    {
        // TODO
    }

    fn exec_new_process(&self)
    {
        // TODO
        println!("TODO: exec_new_process()");
    }
    */

    /*
    fn wait_syscall_entry(&mut self) {
        let (data, src) = self.protocol.receive().expect("Fail to receive data");

        let message = String::from_utf8_lossy(&data);
        println!("[EXECUTOR: RECEIVE] {}", message);

        self.syscall = serde_json::from_str(&message).expect("Fail to parse JSON");
    }

    fn reply_syscall_exit(&mut self) {
        println!("[EXECUTOR: REPLY] ack exit");
        self.protocol.reply("ack exit").unwrap();
    }
    */

    fn log_syscall(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[EXECUTOR] {}", json)
    }

}
