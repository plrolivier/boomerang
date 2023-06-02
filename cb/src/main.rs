/*
 * Example to use libsysforward with a tracer and an executor.
 */
use std::{
    process::{Command, Stdio },
};


/*
 * Start the tracer and executor 2 different processes, 
 * and wait for them to finish.
 */
 fn main() {


    let mut tracer = Command::new("./target/debug/ptracer")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failt to spawn the tracer");
    println!("Spawn tracer with PID: {}", tracer.id());


    let mut executor = Command::new("./target/debug/executor")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failt to spawn the executor");
    println!("Spawn executor with PID: {}", executor.id());


    tracer.wait().expect("Error while waiting for tracer");
    executor.wait().expect("Error while waiting for executor");

}
