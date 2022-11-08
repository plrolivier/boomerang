/*
 * The lib API
 */

mod operation;

mod protocol;

pub mod arch;
pub mod tracer_engine;
pub mod executor_engine;

pub use crate::tracer_engine::Tracer;
pub use crate::executor_engine::Executor;




/*
 * TODO: Find another place to put Syscall structures...
 */
use crate::{
    tracer_engine::{
        decoder::{ Decoder, Decode },
        filtering::{ Decision },
    },
};



struct RawSyscall {
    no: u64,
    args: Vec<u64>,
    retval: u64,
    errno: u64,
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

    fn to_json(&self) -> String {
        format!("{{\"no\": {}, \"args\": {:?}, \"retval\": {}, \"errno\": {}}}", self.no, self.args, self.retval, self.errno)
    }
}


pub struct Syscall {
    raw: RawSyscall,
    name: String,
    args: Vec<Option<Box<dyn Decode>>>,
    decision: Option<Decision>,
}

impl Syscall {
    fn new() -> Self {
        Self {
            raw: RawSyscall::new(),
            name: String::with_capacity(25),
            //args: vec![&None; 7],
            args: Vec::from([None, None, None, None, None, None, None]),
            //decision: None,
            decision: Some(Decision::Continue), //Once the filtering implemented, put None 
        }
    }

    fn print(&self) {
        for arg in self.args.iter() {
            match arg {
                Some(a) => a.print(),
                None => break,
            }
        }
    }

    fn args_to_json(&self) -> String {
        // TODO: improve format here
        let mut s = String::new();
        s.push('[');
        for arg in self.args.iter() {
            match arg {
                Some(a) => s.push_str(&a.to_json()),
                None => break,
            }
            s.push(',');    //TODO: always add a trailing comma...
        }
        s.push(']');
        s
    }

    fn to_json(&self) -> String {
        // TODO: replace 0 with self.decision
        format!("{{\"raw\": {}, \"name\": \"{}\", \"args\": {}, \"decision\": {:?}}}", self.raw.to_json(), self.name, self.args_to_json(), self.decision)
    }

}

