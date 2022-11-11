/*
 * TODO: use eBPF to filter syscall and output the decision.
 */
use serde::{Serialize, Deserialize};
use crate::tracer_engine::Syscall;



#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum Decision {
    Continue    = 0,
    FwdEntry    = 1,
    FwdExit     = 2,
    InspectExit = 3,
    LogLocal    = 4,
    NoExec      = 5,
    Kill        = 6,
}

pub struct Filter {
    pub name: String,
    pub decision: Decision,
}

impl Filter {
    pub fn new(name: String) -> Filter {
        Filter {
            name: name,
            decision: Decision::Continue,
        }
    }

    pub fn filter(&self, syscall: &Syscall) -> Decision {
        let mut decision = Decision::Continue;

        if syscall.entry_decoded {
            match syscall.name {
                _ => { decision = self.decision; },
            }

        } else {
            match syscall.raw.no {
                _ => { decision = self.decision; },
            }
        }

        decision
    }
}
