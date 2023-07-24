/*
 * The filter is composed of multiple rules checks one after the other.
 * The library shares a Rule trait which can be implemented by the debugger to satisfy its needs.
 *
 * In the future, it would be nice to support eBPF filter in a similar way as seccomp does.
 * But what would it brings more than the Rule trait?
 */
use serde::{Serialize, Deserialize};
use crate::tracer::Syscall;



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

    /*
     * The filter functions called after the decoder on syscall entry and exit.
     */
    fn filter_entry(&mut self, syscall: Syscall) -> Result<Decision, std::io::Error>;
    fn filter_exit(&mut self, syscall: Syscall) -> Result<Decision, std::io::Error>;

    /*
     * A callback called on syscall exit after the library instrumentation so that the filter can be updated at runtime.
     * E.g., to keep track of file descriptors during an execution
     */
    fn on_syscall_exit(&mut self, syscall: Syscall);
}



pub struct Filter {
    pub name: String,
    rules: Vec<Box<dyn Rule>>,
    default_decision: Decision,
}

impl Filter {

    pub fn new(name: String) -> Self
    {
        Filter {
            name: name,
            rules: Vec::new(),
            default_decision: Decision::Continue,
        }
    }

    pub fn insert(&mut self, index: usize, rule: Box<dyn Rule>)
    {
        self.rules.insert(index, rule)
    }

    pub fn remove(&mut self, index: usize) -> Box<dyn Rule>
    {
        self.rules.remove(index)
    }

    /*
     * Return the decision made by the first rule to match,
     * otherwise returns the default decision.
     */
    pub fn filter(&mut self, insyscall: bool, syscall: &Syscall) -> Decision
    {
        let mut decision: Option<Decision> = None;
        // = self.default_decision;

        for rule in self.rules.iter_mut() {

            // We clone to ensure each rule has the correct syscall and has not been modified by the user.
            let clone_syscall = syscall.clone();

            let result = match insyscall {
                false => rule.filter_entry(clone_syscall),
                true => rule.filter_exit(clone_syscall),
            };

            match result {
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

    /*
     * Execute rule callbacks.
     *
     */
    pub fn on_syscall_exit(&mut self, syscall: &Syscall)
    {
        for rule in self.rules.iter_mut() {
            let clone_syscall = syscall.clone();
            rule.on_syscall_exit(clone_syscall)
        }
    }

}

