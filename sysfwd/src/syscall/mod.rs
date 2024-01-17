/*
 * Syscall data structures
 */
pub mod args;
pub mod decoder;
pub mod encoder;
pub mod syscalls;


use serde::{ Serialize, Deserialize };

use crate::{
        syscall::decoder::DecodedSyscall,
        tracer::filtering::Decision,
};



/*
 * TODO: add timestamps for calculating how much time the syscall spent in kernel / forwarding.
 */

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Syscall {
    pub raw: RawSyscall,
    /* I chose to represent the DecodedSyscall with an Enum instead of a Trait
     *  because it is easier to (de)serialize
     */
    pub decoded: Option<DecodedSyscall>,
    //pub args: Vec<Option<Box<dyn Decode>>>,         // TODO: replace with Option<T>...
    pub name: String,
    pub decision: Option<Decision>,
}

impl Syscall {
    pub fn new() -> Self {
        Self {
            raw: RawSyscall::new(),
            decoded: None,
            name: String::with_capacity(25),
            decision: Some(Decision::Continue),     // Once the filtering implemented, put None 
        }
    }

}


/* 
 * A raw syscall represents the raw intercepted values 
 * 
 * TODO: change attribute to private and use methods to synchronized with DecodedSyscall if the syscall is decoded...
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct RawSyscall {
    pub no: usize,
    pub args: Vec<usize>,
    pub retval: usize,
    pub errno: usize,
}

impl RawSyscall {
    pub fn new() -> Self {
        Self {
            no: 0,
            args: vec![0; 7],
            retval: 0,
            errno: 0,
        }
    }

    /* 
    pub fn from_x86_exit(regs: user_regs_struct, rawsyscall: &RawSyscall) -> Self
    {
        let mut new_raw = rawsyscall.clone();

        // assert scno == orig_rax

        new_raw.retval = regs.rax as usize;
        new_raw.errno = regs.rdx as usize;
        new_raw
    }
    */
}
