/*
 * The tracer engine takes care of handling syscalls.
 */
mod tracer_engine;
pub mod file_descriptor;
pub mod filtering;

pub use tracer_engine::TracerEngine;


use std::io;
use nix::{
    unistd::Pid,
    sys::signal::Signal,
};
use crate::syscall::Syscall;



/*
 * The tracer trait describes the interface a debugger should implement to be compatible with the control channel
 * and controlled by avatar2.
 */
pub trait TracerCallback {
    fn spawn_process(&mut self, program: String, prog_args: Vec<String>) -> Result<Pid, io::Error>;
    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>;
    fn start_tracing(&mut self, pid: Pid) -> Result<(), io::Error>;
    fn cont_tracing(&mut self, pid: Pid, signal: Option<Signal>) -> Result<(), io::Error>;
    fn stop_tracing(&mut self, pid: Pid) -> Result<(), io::Error>;
}

