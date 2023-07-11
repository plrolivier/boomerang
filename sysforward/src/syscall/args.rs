/*
 * Syscall decoded arguments data structures
 */
use core::fmt;

//use nix::libc::printf;
use serde::{ Serialize, Deserialize };

use crate::{
    operation::{ Operation },
    tracer::{
        decoder::{ Decode },
    },
};



#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum ArgType {
    Integer(Integer),
    Fd(Fd),
    Size(Size),
    Offset(Offset),
    Flag(Flag),
    Protection(Protection),
    Signal(Signal),
    Address(Address),
    Buffer(Buffer),
    NullBuffer(NullBuffer),     // i.e., string
    Array(Array),
    Struct(Struct),
}

impl Decode for ArgType {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) {
        match self {
            ArgType::Integer(integer)   => integer.decode(pid, operation),
            ArgType::Fd(fd)                  => fd.decode(pid, operation),
            ArgType::Size(size)            => size.decode(pid, operation),
            ArgType::Offset(offset)      => offset.decode(pid, operation),
            ArgType::Flag(flag)            => flag.decode(pid, operation),
            ArgType::Protection(protection) => protection.decode(pid, operation),
            ArgType::Signal(signal)      => signal.decode(pid, operation),
            ArgType::Address(address)   => address.decode(pid, operation),
            ArgType::Buffer(buffer)      => buffer.decode(pid, operation),
            ArgType::NullBuffer(nullbuffer) => nullbuffer.decode(pid, operation),
            ArgType::Array(array)         => array.decode(pid, operation),
            ArgType::Struct(structure) => structure.decode(pid, operation),
        }
    }
}


/* 
 * Direct value 
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Integer {
    pub value: u64
}

impl Integer {
    pub fn new(value: u64) -> Self {
        Self { value: value }
    }
}

impl Decode for Integer { 
    fn print(&self) {
        println!("integer: {:#x}", self.value);
    }
}


/* 
 * File descriptor
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Fd {
    pub value: u16,
}

impl Fd {
    pub fn new(value: u64) -> Self {
        Self { value: value as u16 }
    }
}

impl Decode for Fd {

    fn print(&self) {
        println!("fd: {:#x}", self.value);
    }
}


/* 
 * Represent size_t 
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Size {
    pub value: u64,
    //pub obj: Option<T>,
    
}

impl Size {
    pub fn new(value: u64) -> Self {
        Self { 
            value: value,
        }
    }
}

impl Decode for Size {
    fn print(&self) {
        println!("size: {:#x}", self.value);
    }
}


/*
 * Represent offset_t
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Offset {
    pub value: u64,
}

impl Offset {
    pub fn new(value: u64) -> Self {
        Self {
            value: value,
        }
    }
}

impl Decode for Offset {
    fn print(&self) {
        println!("offset: {:#x}", self.value);
    }
}


/*
 * Represent a flag
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Flag {
    pub value: u8,
}

impl Flag {
    pub fn new(value: u64) -> Self {
        Self { value: value as u8 }
    }
}

impl Decode for Flag {
    fn print(&self) {
        println!("flag: {:#x}", self.value);
    }
}


/*
 * Represent a memory protection
 * XXX: Should it be replace with an enum?
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Protection {
    pub value: u8,
}

impl Protection {
    pub fn new(value: u64) -> Self {
        Self { value: value as u8 }
    }
}

impl Decode for Protection {

    fn print(&self) {
        println!("protection: {:#x}", self.value);
    }
}


/*
 * Use for signal number
 * XXX: Should we use an enum?
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
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

    fn print(&self) {
        println!("signo: {:#x}", self.value);
    }
}



/* 
 * Pointers: value used to point information in memory 
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum Direction {
    In  = 0x1,
    Out = 0x2,
    InOut = 0x3,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::In => write!(f, "IN"),
            Direction::Out => write!(f, "OUT"),
            Direction::InOut => write!(f, "IN/OUT"),
        }
    }
}

impl fmt::LowerHex for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = match self {
            Direction::In => "1",
            Direction::Out => "2",
            Direction::InOut => "3",
        };
        write!(f, "{}", hex)
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Address {
    pub value: u64,
    pub direction: Direction,
}

impl Address {
    pub fn new(value: u64, direction: Direction) -> Self {
        Self { 
            value: value,
            direction: direction
        }
    }
}

impl Decode for Address {
    fn print(&self) {
        println!("address: {:#x}", self.value);
        println!("direction: {:#x}", self.direction);
    }
}



/* Pointer arguments */

/*
 * A memory block
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Buffer {
    pub address: u64,
    pub direction: Direction,
    pub size: u64,
    pub content: Vec<u8>,
}

impl Buffer {
    pub fn new(address: u64, direction: Direction, size: u64) -> Self {
        Self { 
            address: address,
            direction: direction,
            size: size,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}

impl Decode for Buffer {

    fn decode(&mut self, pid: i32, operation: &Box<Operation>) { 
        self.content = operation.memory.read(pid, self.address, self.size);
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("direction: {:#x}", self.direction);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}


/*
 * A null-terminated memory block, also known as a string
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct NullBuffer {
    pub address: u64,
    pub direction: Direction,
    pub size: u64,
    pub content: Vec<u8>,
}

impl NullBuffer {
    pub fn new(address: u64, direction: Direction) -> Self {
        Self { 
            address: address,
            direction: direction,
            size: 0,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}

impl Decode for NullBuffer {

    fn decode(&mut self, pid: i32, operation: &Box<Operation>) { 
        //TODO: does not work when the Null terminated buffer is greater than READ_SIZE bytes.
        #[allow(non_snake_case)]
        let READ_SIZE = 1024;
        let buf = operation.memory.read(pid, self.address, READ_SIZE);

        let mut iter = buf.iter();
        loop {
            match iter.next() {
                Some(x) => {
                    match x {
                        0 => break,
                        _ => {
                            self.size += 1;
                            self.content.push(x.clone());
                        },
                    }
                }
                None => break,
            }
        }
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("direction: {:#x}", self.direction);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}


/*
 * Represent an array of value
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Array {
    pub address: u64,
    pub direction: Direction,
    pub count: u64,
    pub content: Vec<u32>,
    /* XXX
    pub element_type: Option<T>,
    pub element_size: u64,
    */
}

impl Array {
    pub fn new(address: u64, direction: Direction, count: u64) -> Self {
        Self {
            address: address,
            direction: direction,
            count: count,
            content: Vec::new(),
        }
    }
}

impl Decode for Array {

    fn decode(&mut self, _pid: i32, _operation: &Box<Operation>) { 
       panic!("To implement"); 
    }
}

/*
 * Represent a structure in memory
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Struct {
    pub address: u64,
    pub direction: Direction,
    pub size: u64,
    pub name: String,
    pub content: Vec<u8>,
}

impl Struct {
    //pub fn new(address: u64, name: &str) -> Self {    TODO: use name during creation
    pub fn new(address: u64, direction: Direction) -> Self {
        Self { 
            address: address,
            direction: direction,
            size: 0,
            //name: name.to_string(),
            name: String::new(),
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}

impl Decode for Struct {

    fn decode(&mut self, _pid: i32, _operation: &Box<Operation>) { 
       panic!("To implement"); 
    }

    fn print(&self) {
        println!("name: {}", self.name);
        println!("direction: {}", self.direction);
        println!("size: {:#x}", self.size);
        println!("address: {:#x}", self.address);
        println!("content: {:#x?}", self.content);
    }
}