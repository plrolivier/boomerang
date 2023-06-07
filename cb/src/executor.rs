/*
 *
 */
use sysforward::{
    arch::TargetArch,
    executor_engine::Executor,
};

use crate::{ IP_ADDRESS, TRACER_PORT, EXECUTOR_PORT };



pub fn start_executor()
{
    println!("[EXECUTOR] Start listening on {}:{}", IP_ADDRESS, EXECUTOR_PORT);

    let mut executor = Executor::new(TargetArch::X86_64, IP_ADDRESS, EXECUTOR_PORT, TRACER_PORT);

    executor.run();

    println!("[EXECUTOR] Stop executing.");
}
