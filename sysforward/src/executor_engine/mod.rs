/*
 *
use crate::{
    arch::{ TargetArch, Architecture },
    protocol::{ Command, Packet, Header, SendSyscallEntryPayload, Server },
};



pub struct Executor {
    pub arch: Architecture,
    connection: Server,
}

impl Executor {

    pub fn new(target_arch: TargetArch) -> Self {
        Self {
            arch: Architecture::new(target_arch),
            connection: Server{},
        }
    }

    pub fn run(&self) {
        //let stream = self.connection.listen();
        self.connection.listen();
    }
}

*/