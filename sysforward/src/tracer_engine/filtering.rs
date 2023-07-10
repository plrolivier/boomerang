/*
 * TODO: create a trait which does the filtering and can be implemented by the target.
 * TODO: use eBPF to filter syscall and output the decision.
 */
use serde::{Serialize, Deserialize};
use crate::tracer_engine::Syscall;



#[derive(PartialEq)]
#[derive(Clone, Copy, Debug)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum Decision {
    /*
     * Do not consider this rule and pass to the next
     */
    Pass        = 0,
    /*
     * Continue the syscall execution without further instrumentation
     */
    Continue    = 1,
    /* 
     * On syscall entry, remotly execute the syscall with executor 
     * and replace the return value on syscall exit
     */
    Forward     = 2,
    /*
     * Inspect the syscall return value on exit
     */
    Inspect     = 3,
    /*
     * Replace the syscall with a dummy one
     */
    NoExec      = 4,
    /*
     * Terminate the process
     */
    Kill        = 5,
}



pub trait Rule {
    fn filter(&self, syscall: &Syscall) -> Result<Decision, std::io::Error>;
}



pub struct Filter {
    pub name: String,
    //pub decision: Decision,
    rules: Vec<Box<dyn Rule>>,
    default_decision: Decision,
}

impl Filter {
    pub fn new(name: String) -> Self {
        Filter {
            name: name,
            rules: Vec::new(),
            default_decision: Decision::Continue,
        }
    }

    /*
     * Return the decision made by the first rule to match,
     * otherwise returns the default decision.
     */
    pub fn filter(&self, syscall: &Syscall) -> Decision {
        let mut decision: Option<Decision> = None;
        // = self.default_decision;

        for rule in &self.rules {
            match rule.filter(syscall) {
                Ok(result) => {
                    if result != Decision::Pass {
                        decision = Some(result);
                        break;
                    }
                },
                Err(_) => continue,
            }
        }

        if decision == None {
            return self.default_decision;
        }
        return decision.unwrap();
    }

    pub fn insert(&mut self, index: usize, rule: Box<dyn Rule>)
    {
        self.rules.insert(index, rule)
    }

    pub fn remove(&mut self, index: usize) -> Box<dyn Rule>
    {
        self.rules.remove(index)
    }

}

