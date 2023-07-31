/*
 * The interfaces used by the tracer engine to interact with the tracee process.
 */
use core::ffi::c_void;
use std::io;

use nix::{
    unistd::Pid,
    libc::user_regs_struct,
    sys::ptrace,
};
use crate::operation::{ RegisterOperation, MemoryOperation };



#[derive(Clone, Debug)]
pub struct Ptrace { }

impl RegisterOperation for Ptrace {

    fn read_registers(&self, pid: i32) -> Option<user_regs_struct> {
        let pid = Pid::from_raw(pid);
        Some(ptrace::getregs(pid).unwrap())
    }

    fn write_registers(&self, pid: i32, regs: user_regs_struct) -> Result<(), io::Error> {
        let pid = Pid::from_raw(pid);
        match ptrace::setregs(pid, regs) {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("[{}] Error setting registers: {}", pid, e);
                return Err(e.into())
            },
        }
    }
}

impl MemoryOperation for Ptrace {

    fn read(&self, pid: i32, addr: usize, size: usize) -> Vec<u8> {
        let pid = Pid::from_raw(pid);
        let mut mem: Vec<u8> = Vec::new();
        let mut addr = addr;
        let mut count = size + (4 - size % 4);
        
        /*
        println!("On process {}, read {} at {:#x}", pid, size, addr);
        let pid_u32 = pid.as_raw() as u32;
        let maps = read_process_memory_maps(pid_u32);
        print_memory_regions(&maps);
        */

        while count > 0 {
            let address = addr as ptrace::AddressType;
            //mem.push(ptrace::read(pid, address).unwrap() as u32);
            //let word = ptrace::read(pid, address).unwrap() as u32;
            let word: u32;
            match ptrace::read(pid, address) {
                Ok(w) => {
                    word = w as u32;
                }
                Err(err)=> {
                    eprintln!("An error {} occured during read at {:?} on {}", err, address, pid);
                    break;
                }
            }
            mem.extend_from_slice(&word.to_le_bytes());
            addr += 4;
            count -= 4;
            //println!("Read returns {}, remains {} bytes", word, count);
        }
        mem
    }

    fn write(&self, pid: i32, addr: usize, mem: Vec<u8>) -> usize
    {
        let pid = Pid::from_raw(pid);
        let mut addr = addr;
        let mut mem = mem;

        // Pad the memory to be a multiple of 4
        let pad_bytes = 4 - (mem.len() % 4);
        mem.extend(std::iter::repeat(0).take(pad_bytes));

        //println!("[WRITE] {:?}", mem);

        let size = mem.len();
        let mut count = mem.len();

        for chunk in mem.chunks(4) {

            let address = addr as ptrace::AddressType;
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let word = word as *mut c_void;

            unsafe {
                match ptrace::write(pid, address, word) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("An error {} occured during write at {:?} on {}", err, address, pid);
                        break;
                    }
                }
            }
            addr += 4;
            count -= 4;
        }

        size - count
    }

}
