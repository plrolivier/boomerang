/*
 *
 */
use std::{
    thread,
};
use sysforward::{
    arch::TargetArch,
    executor_engine::Executor,
};



fn start_executor() {
    println!("[executor] Start listening...");
    //thread::sleep(std::time::Duration::from_secs(2));

    let mut executor = Executor::new(TargetArch::X86_64);

    executor.run();
}


fn main() {
    start_executor();
}