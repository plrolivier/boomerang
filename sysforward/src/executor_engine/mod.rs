/*
 *
 */
use crate::{
    arch::{ TargetArch, Architecture },
    syscall::{ Syscall },
    network::udpserver::UdpServer,
};



pub struct Executor {
    pub arch: Architecture,
    syscall: Syscall,
    connection: UdpServer,
}

impl Executor {

    pub fn new(target_arch: TargetArch) -> Self {
        Self {
            arch: Architecture::new(target_arch),
            syscall: Syscall::new(),
            connection: UdpServer::new().expect("Fail to create server"),
        }
    }

    pub fn run(&mut self) {

        loop {
            self.wait_syscall();
            self.reply_syscall_entry();
            self.log_entry();

            self.wait_syscall();
            self.reply_syscall_exit();
            self.log_exit();
        }
    }

    fn wait_syscall(&mut self) {
        let (data, src) = self.connection.receive().expect("Fail to receive data");

        let message = String::from_utf8_lossy(&data);
        println!("[EXECUTOR: RECEIVE] {}", message);

        self.syscall = serde_json::from_str(&message).expect("Fail to parse JSON");
    }


    fn reply_syscall_entry(&mut self) {
        println!("[EXECUTOR: REPLY] ack entry");
        self.connection.reply("ack entry").unwrap();
    }

    fn reply_syscall_exit(&mut self) {
        println!("[EXECUTOR: REPLY] ack exit");
        self.connection.reply("ack exit").unwrap();
    }


    fn log_entry(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[EXECUTOR] {}", json)
    }

    fn log_exit(&self) {
        let json = serde_json::to_string(&self.syscall).unwrap();
        println!("[EXECUTOR] {}", json);
        println!("");
    }

}
