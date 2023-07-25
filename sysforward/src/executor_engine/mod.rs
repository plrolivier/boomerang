/*
 *
 */

mod executor_engine;


pub use executor_engine::ExecutorEngine;



use std::{
    io,
};
use nix::{
    unistd::{ Pid },
};



pub trait ExecutorCallback {
    fn spawn_process(&mut self, program: &str, prog_args: &[&str]) -> Result<Pid, io::Error>;
    fn kill_process(&mut self, pid: Pid) -> Result<(), io::Error>;
}