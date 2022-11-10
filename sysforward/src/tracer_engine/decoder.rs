/*
 * Decode syscall arguments.
 */
use std::rc::Rc;
use std::any::Any;
use serde::{Serialize, Deserialize};
use crate::{
    arch::{ TargetArch, Architecture },
    syscall::Syscall,
    operation::Operation,
};



pub struct Decoder {
    arch: Rc<Architecture>,
}

impl Decoder {
    pub fn new(arch: Rc<Architecture>) -> Self {
        Self { 
            //arch: Architecture::new(arch),
            arch: arch,
        }
    }

    pub fn decode_entry(&self, syscall: &mut Syscall, pid: i32, operation: &Box<dyn Operation>) {

        // TODO: improve the match by using number instead of strings
        match self.arch.syscall_table.get_syscall_name(&syscall.raw.no) {
            Some(x) => syscall.name = x,
            None => println!("No name found for {}", syscall.raw.no),
        }

        /*
         * First, assign a type to each argument according to the syscall.
         */
        self.parse_args(syscall);

        /*
         * Second, iterate over the argument to decode them.
         */
        self.decode_args(syscall, pid, operation);
    }

    fn decode_args(&self, syscall: &mut Syscall, pid: i32, operation: &Box<dyn Operation>) {
        for arg in syscall.args.iter_mut() {
            match arg {
                Some(x) => x.decode(pid, operation),
                None => break,
            }
        }
    }

    fn parse_args(&self, syscall: &mut Syscall) {

        match syscall.name.as_str() {
            "open" => {
                syscall.args[0] = Some(Box::new(NullBuf::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(Flag::new(syscall.raw.args[1])));
                syscall.args[2] = Some(Box::new(Int::new(syscall.raw.args[2])));
            },
            "openat" => {
                syscall.args[0] = Some(Box::new(Int::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(NullBuf::new(syscall.raw.args[1])));
                syscall.args[2] = Some(Box::new(Flag::new(syscall.raw.args[2])));
                syscall.args[3] = Some(Box::new(Int::new(syscall.raw.args[3])));
            },
            "read" => {
                syscall.args[0] = Some(Box::new(Fd::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(Buffer::new(syscall.raw.args[1], syscall.raw.args[2])));
                syscall.args[2] = Some(Box::new(Size::new(syscall.raw.args[2])));
            },
            "write" => {
                syscall.args[0] = Some(Box::new(Fd::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(Buffer::new(syscall.raw.args[1], syscall.raw.args[2])));
                syscall.args[2] = Some(Box::new(Size::new(syscall.raw.args[2])));
            },
            "close" => {
                syscall.args[0] = Some(Box::new(Fd::new(syscall.raw.args[0])));
            },
            "mmap" => {
                syscall.args[0] = Some(Box::new(Address::new(syscall.raw.args[0])));
                syscall.args[1] = Some(Box::new(Size::new(syscall.raw.args[1])));
                syscall.args[2] = Some(Box::new(Prot::new(syscall.raw.args[2])));
                syscall.args[3] = Some(Box::new(Fd::new(syscall.raw.args[3])));
                syscall.args[4] = Some(Box::new(Offset::new(syscall.raw.args[4])));
            },
            _ => (),
        }
    }


    pub fn decode_exit(&self) { }
}



pub trait Decode: CloneDecode {
    fn as_any(&self) -> &dyn Any;
    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) { }
    fn to_json(&self) -> String { format!("{{\"type\": none, \"value\": 0}}") }
    fn print(&self) { }
}

/*
 * https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7
 */
pub trait CloneDecode {
    fn clone_decode<'a>(&self) -> Box<dyn Decode>;
}

impl<T> CloneDecode for T
where T: Decode + Clone + 'static, {
    fn clone_decode(&self) -> Box<dyn Decode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Decode> {
    fn clone(&self) -> Self {
        self.clone_decode()
    }
}

/* 
 * Direct value 
 */
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Int { pub value: u64 }
impl Int {
    pub fn new(value: u64) -> Self {
        Self { value: value }
    }
}
impl Decode for Int { 
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"integer\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("value: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Fd { pub value: u16 }
impl Fd {
    pub fn new(value: u64) -> Self {
        Self { value: value as u16 }
    }
}
impl Decode for Fd {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"fd\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("fd: {:#x}", self.value);
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Size { pub value: u64 }
impl Size {
    pub fn new(value: u64) -> Self {
        Self { value: value }
    }
}
impl Decode for Size {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"size\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("size: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Offset { pub value: u64 }
impl Offset {
    pub fn new(value: u64) -> Self {
        Self { value: value }
    }
}
impl Decode for Offset {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"offset\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("offset: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Flag { pub value: u8 }
impl Flag {
    pub fn new(value: u64) -> Self {
        Self { value: value as u8 }
    }
}
impl Decode for Flag {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"flag\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("flags: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Prot { pub value: u8 }
impl Prot {
    pub fn new(value: u64) -> Self {
        Self { value: value as u8 }
    }
}
impl Decode for Prot {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"prot\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("prot: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Signal {
    pub value: u8,
    /*
    sig: NixSignal,
    */
}
impl Signal {
    pub fn new(value: u64) -> Self {
        Self { value: value as u8 }
    }
}
impl Decode for Signal {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"signal\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("signo: {:#x}", self.value);
    }
}


/* 
 * Pointers 
 */
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Address {
    pub value: u64
}
impl Address {
    pub fn new(value: u64) -> Self {
        Self { value: value }
    }
}
impl Decode for Address {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"address\", \"value\": {}}}", self.value)
    }

    fn print(&self) {
        println!("address: {:#x}", self.value);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Buffer {
    pub address: u64,
    pub size: u64,
    pub content: Vec<u32>,
}
impl Buffer {
    pub fn new(address: u64, size: u64) -> Self {
        Self { 
            address: address,
            size: size,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}
impl Decode for Buffer {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) { 
        self.content = operation.read_memory(pid, self.address, self.size);
    }

    fn to_json(&self) -> String {
        format!("{{\"type\": \"buffer\", \"value\": {{\"address\": {}, \"size\": {}, \"content\": {:?}}}}}", self.address, self.size, self.content)
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct NullBuf {
    pub address: u64,
    pub size: u64,
    pub content: Vec<u32>,
}
impl NullBuf {
    pub fn new(address: u64) -> Self {
        Self { 
            address: address,
            size: 0,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}
impl Decode for NullBuf {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn decode(&mut self, pid: i32, operation: &Box<dyn Operation>) { 
        //TODO: does not work when the Null terminated buffer is greater than READ_SIZE bytes.
        let READ_SIZE = 1024;
        let buf = operation.read_memory(pid, self.address, READ_SIZE);

        let mut iter = buf.iter();
        loop {
            match iter.next() {
                Some(x) => {
                    match x {
                        _ => {
                            self.size += 1;
                            self.content.push(x.clone());
                        },
                        0 => break,
                    }
                }
                None => break,
            }
        }
    }

    fn to_json(&self) -> String {
        format!("{{\"type\":\"buffer\",\"value\":{{\"address\":{},\"size\":{},\"content\":{:?}}}}}", self.address, self.size, self.content)
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Struct {
    pub address: u64,
    pub size: u64,
    pub name: String,
    pub content: Vec<u32>,
}
impl Struct {
    pub fn new(address: u64, name: &str) -> Self {
        Self { 
            address: address,
            size: 0,
            name: name.to_string(),
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}
impl Decode for Struct {
    fn as_any(&self) ->&dyn Any {
        self
    }

    fn to_json(&self) -> String {
        format!("{{\"type\":\"struct\",\"value\":{{\"address\":{},\"size\":{},\"name\":{},\"content\":{:?}}}}}", self.address, self.size, self.name, self.content)
    }

    fn print(&self) {
        println!("name: {}", self.name);
        println!("size: {:#x}", self.size);
        println!("address: {:#x}", self.address);
        println!("content: {:#x?}", self.content);
    }
}

