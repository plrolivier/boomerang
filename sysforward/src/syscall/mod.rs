/*
 * Syscall data structures
 */
pub mod args;
//pub mod filesystem;

pub mod open;
pub mod io;
pub mod ioctl;
pub mod access;
pub mod fallocate;
pub mod truncate;
pub mod renameat;
pub mod memfd;
pub mod mknod;
pub mod file_handle;
pub mod mmap;
pub mod execve;
pub mod prctl;
pub mod dirent;
pub mod stat;
pub mod resource;


use serde::{ Serialize, Deserialize };

use crate::{
    tracer_engine::{
        decoder::{ DecodedSyscall },
        filtering::{ Decision },
    },
};



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
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct RawSyscall {
    pub no: u64,
    pub args: Vec<u64>,
    pub retval: u64,
    pub errno: u64,
}

impl RawSyscall {
    fn new() -> Self {
        Self {
            no: 0,
            args: vec![0; 7],
            retval: 0,
            errno: 0,
        }
    }
}
