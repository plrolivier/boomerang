/*
 * Syscall data structures
 */
pub mod args;
pub mod filesystem;


//use core::fmt;
//use std::any::Any;

use serde::{ Serialize, Deserialize };

use crate::{
    //operation::{ Operation },
    tracer_engine::{
        decoder::{ DecodedSyscall },
        filtering::{ Decision },
    },
};



/*
macro_rules! decode_integer {
    ($n:expr, $type:ident) => {
        syscall.args.push(SyscallArg::$type($type { value: syscall.raw.args[$n] }))
    }
}

macro_rules! decode_buffer {
    ($n:expr, $m:expr) => {
        syscall.args.push(Box::new(Buffer::new(syscall.raw.args[$n], syscall.raw.args[$m])))
    }
}

macro_rules! decode_string {
    ($n:expr, $d:expr) => {
        syscall.args.push(SyscallArg::NullBuffer(NullBuffer { 
            address: syscall.raw.args[$n],
            direction: $d
        }))
    }
}
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
    //pub entry_decoded: bool,
    //pub args: Vec<SyscallArg>,
    pub name: String,
    pub decision: Option<Decision>,
}

impl Syscall {
    pub fn new() -> Self {
        Self {
            raw: RawSyscall::new(),
            decoded: None,
            //entry_decoded: false,
            //args: Vec::new(),
            name: String::with_capacity(25),
            decision: Some(Decision::Continue), //Once the filtering implemented, put None 
        }
    }

    /*
    fn print(&self) {
        for arg in self.args.iter() {
            match arg {
                Some(a) => a.print(),
                None => break,
            }
        }
    }
    */

    /*
    fn args_to_json(&self) -> String {
        // TODO: improve format here
        //let mut s = String::new();
        //s.push('[');
        //for arg in self.args.iter() {
            //s.push_str(&arg.to_json());
            ////match arg {
                ////Some(a) => s.push_str(&a.to_json()),
                ////None => break,
            ////}
            //s.push(',');    //TODO: always add a trailing comma...
        //}
        //s.push(']');
        //s
        let args_json = self.args.iter().map(|arg| arg.to_json()).collect::<Vec<_>>();
        json!({
            "args": args_json,
        }).to_string()
    }
    */

    //pub fn to_json(&self) -> String {
    //    // TODO: replace 0 with self.decision
    //    format!("{{\"raw\": {}, \"name\": \"{}\", \"args\": {}, \"decision\": {:?}}}",
    //            self.raw.to_json(), self.name, self.args_to_json(), self.decision.unwrap() as u8)
    //}

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

    //fn to_json(&self) -> String {
    //    format!("{{\"no\": {}, \"args\": {:?}, \"retval\": {}, \"errno\": {}}}", self.no, self.args, self.retval, self.errno)
    //}
}
