/*
 *
 */
use crate::{
    syscall::{ Syscall },
};

pub struct Invoker {
    syscall: Syscall,
}

impl Invoker 
{
    pub fn new() -> Self
    {
        Self {
            syscall: Syscall::new(),
        }
    }

    pub fn invoke_syscall(&self, syscall: Syscall)
    {
        // TODO
    }

    pub fn invoke_new_process(&self)
    {
        // TODO
        println!("TODO: invoke_new_process()");
    }
}