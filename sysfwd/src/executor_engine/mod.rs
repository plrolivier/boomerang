/*
 *
 */

mod executor_engine;

pub use executor_engine::ExecutorEngine;


use std::{
    io,
    process::Child,
};
use nix::unistd::Pid;



pub trait ExecutorCallback {
    fn spawn_process(&mut self, program: &str, prog_args: &[&str]) -> Result<Pid, io::Error>;
    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>;
}


pub trait Invoker {
    fn invoke_syscall(&self, scno: usize, arg1:usize, arg2: usize,
                      arg3: usize, arg4: usize, arg5: usize, arg6: usize,
                      arg7: usize) -> Result<(usize, usize), io::Error>;
    fn invoke_new_process(&self) -> Result<Child, io::Error>;
}