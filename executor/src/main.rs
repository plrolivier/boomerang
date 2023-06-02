/*
 *
 */
use std::{
    env,
};

use sysforward::{
    arch::TargetArch,
    executor_engine::Executor,
};


/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
static TRACER_PORT: u16 = 31000;
static EXECUTOR_PORT: u16 = 31001;



fn main()
{
    // TODO: add more argument to configure the tracer
    let args: Vec<String> = env::args().collect();

    /*
    if args.len() < 2 {
        println!("Usage: ./executor <>");
        return;
    }
    */

    println!("[EXECUTOR] Start listening on {}:{}", IP_ADDRESS, EXECUTOR_PORT);

    let mut executor = Executor::new(TargetArch::X86_64, IP_ADDRESS, EXECUTOR_PORT, TRACER_PORT);

    executor.run();
}