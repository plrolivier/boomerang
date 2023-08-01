/*
 *
 */
use serde::{ Serialize, Deserialize };
use decoding_macro::DecodeExit;
use crate::{
    syscall::RawSyscall,
    syscall::args::{ Direction, Integer, Size, Address },
    syscall::decoder::{ DecodeArg, DecodeEntry, DecodeExit },
    targets::operation::Operation,
};



 // long syscall(SYS_get_robust_list, int pid, struct robust_list_head **head_ptr, size_t *len_ptr);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct GetRobustList {
    pub pid: Integer,
    pub head_ptr: Address,
    pub len_ptr: Address,
    pub retval: Option<Integer>,
}
impl GetRobustList {
    pub fn new(raw: RawSyscall) -> Self {
        let pid = Integer::new(raw.args[0]);
        let head_ptr = Address::new(raw.args[1], Direction::InOut);
        let len_ptr = Address::new(raw.args[2], Direction::InOut);
        let retval = None;
        Self { pid, head_ptr, len_ptr, retval }
    }
}
impl DecodeEntry for GetRobustList {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pid.decode(pid, operation).unwrap();
        self.head_ptr.decode(pid, operation).unwrap();
        self.len_ptr.decode(pid, operation).unwrap();
    }
}


// long syscall(SYS_set_robust_list, struct robust_list_head *head, size_t len);
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[derive(DecodeExit)]
pub struct SetRobustList {
    pub pid: Integer,
    pub head_ptr: Address,
    pub len_ptr: Size,
    pub retval: Option<Integer>,
}
impl SetRobustList {
    pub fn new(raw: RawSyscall) -> Self {
        let pid = Integer::new(raw.args[0]);
        let head_ptr = Address::new(raw.args[1], Direction::In);
        let len_ptr = Size::new(raw.args[2]);
        let retval = None;
        Self { pid, head_ptr, len_ptr, retval }
    }
}
impl DecodeEntry for SetRobustList {
    fn decode_entry(&mut self, pid: i32, operation: &Box<Operation>) {
        self.pid.decode(pid, operation).unwrap();
        self.head_ptr.decode(pid, operation).unwrap();
        self.len_ptr.decode(pid, operation).unwrap();
    }
}
