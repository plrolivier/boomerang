/* 
 * Filesystem system calls related
 */
use serde::{ Serialize, Deserialize };

use crate::{
    syscall::RawSyscall,
    syscall::args::{ ArgType, Direction },
    syscall::args::{ Integer, Fd, Size, Offset, Protection, Signal, Flag, Address, Buffer, NullBuffer, Array, Struct },
    //syscall::args::{ Integer, Fd, Size, Flag, Buffer, NullBuffer, Struct },
    tracer_engine::decoder::{ Decode },
    operation::Operation,
};


/* File operations */







