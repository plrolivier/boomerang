/*
 * Syscall decoded arguments data structures
 */
use core::{fmt, panic};
use std::convert::From;

//use nix::libc::printf;
use serde::{ Serialize, Deserialize };

use crate::{
    operation::{ Operation },
    tracer::{
        decoder::{ DecodeArg },
        encoder::{ EncodeArg },
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

impl DecodeArg for ArgType {
    fn decode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
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


// TODO: implement the PartialEq and Eq traits

/* 
 * Direct value 
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Integer {
    pub value: usize
}

impl Integer {
    pub fn new(value: usize) -> Self {
        Self { value: value }
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self
    {
        Self { value: value }
    }
}

impl DecodeArg for Integer { 
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
    pub value: usize,
}

impl Fd {
    pub fn new(value: usize) -> Self {
        Self { value: value }
    }
}

impl From<usize> for Fd {
    fn from(value: usize) -> Self
    {
        Self { value: value }
    }
}

impl DecodeArg for Fd {

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
    pub value: usize,
    //pub obj: Option<T>,
    
}

impl Size {
    pub fn new(value: usize) -> Self {
        Self { 
            value: value,
        }
    }
}

impl From<usize> for Size {
    fn from(value: usize) -> Self
    {
        Self { value: value }
    }
}

impl DecodeArg for Size {
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
    pub value: usize,
}

impl Offset {
    pub fn new(value: usize) -> Self {
        Self {
            value: value,
        }
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self
    {
        Self { value: value }
    }
}

impl DecodeArg for Offset {
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
    pub fn new(value: usize) -> Self {
        Self { value: value as u8 }
    }
}

impl From<usize> for Flag {
    fn from(value: usize) -> Self
    {
        let bytes = value.to_ne_bytes();
        Self { value: bytes[0] }
    }
}

impl DecodeArg for Flag {
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
    pub fn new(value: usize) -> Self {
        Self { value: value as u8 }
    }
}

impl From<usize> for Protection {
    fn from(value: usize) -> Self
    {
        let bytes = value.to_ne_bytes();
        Self { value: bytes[0] }
    }
}

impl DecodeArg for Protection {

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
    pub fn new(value: usize) -> Self {
        Self { value: value as u8 }
    }
}

impl From<usize> for Signal {
    fn from(value: usize) -> Self
    {
        let bytes = value.to_ne_bytes();
        Self { value: bytes[0] }
    }
}

impl DecodeArg for Signal {

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
    pub value: usize,
    pub direction: Direction,
    pub content: usize,
}

impl Address {
    pub fn new(value: usize, direction: Direction) -> Self {
        Self { 
            value: value,
            direction: direction,
            content: 0,
        }
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self
    {
        Self {
            value: value,
            direction: Direction::InOut,
            content: 0,
        }
    }
}

impl DecodeArg for Address {
    fn print(&self) {
        println!("address: {:#x}", self.value);
        println!("direction: {:#x}", self.direction);
        println!("content: {:#x}", self.content);
    }
}

impl EncodeArg for Address {

    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
        // TODO: if content is really used, write it
        Ok(())
    }
}



/* Pointer arguments */

/*
 * A memory block
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Buffer {
    pub address: usize,
    pub direction: Direction,
    pub size: usize,
    pub content: Vec<u8>,
}

impl Buffer {
    pub fn new(address: usize, direction: Direction, size: usize) -> Self {
        Self { 
            address: address,
            direction: direction,
            size: size,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}

impl From<usize> for Buffer {
    fn from(value: usize) -> Self
    {
        Self {
            address: value,
            direction: Direction::InOut,
            size: 0,
            content: Vec::new(),
        }
    }
}

impl DecodeArg for Buffer {

    fn decode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
        self.content = operation.memory.read(pid, self.address, self.size);
        Ok(())
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("direction: {:#x}", self.direction);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}

impl EncodeArg for Buffer {

    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
        let mem = self.content.clone();
        operation.memory.write(pid, self.address, mem);
        Ok(())
    }
}



/*
 * A null-terminated memory block, also known as a string
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct NullBuffer {
    pub address: usize,
    pub direction: Direction,
    pub size: usize,
    pub content: Vec<u8>,
}

impl NullBuffer {
    pub fn new(address: usize, direction: Direction) -> Self {
        Self { 
            address: address,
            direction: direction,
            size: 0,
            content: Vec::new(),  // TODO: initialize with a default size?
        }
    }
}

impl From<usize> for NullBuffer {
    fn from(value: usize) -> Self
    {
        Self {
            address: value,
            direction: Direction::InOut,
            size: 0,
            content: Vec::new(),
        }
    }
}

impl DecodeArg for NullBuffer {

    fn decode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
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
        Ok(())
    }

    fn print(&self) {
        println!("address: {:#x}", self.address);
        println!("direction: {:#x}", self.direction);
        println!("size: {:#x}", self.size);
        println!("content: {:#x?}", self.content);
    }
}

impl EncodeArg for NullBuffer {

    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
        let mem = self.content.clone();
        operation.memory.write(pid, self.address, mem);
        Ok(())
    }
}


/*
 * Represent an array of value
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Array {
    pub address: usize,
    pub direction: Direction,
    pub count: usize,
    pub content: Vec<u32>,
    /* XXX
    pub element_type: Option<T>,
    pub element_size: usize,
    */
}

impl Array {
    pub fn new(address: usize, direction: Direction, count: usize) -> Self {
        Self {
            address: address,
            direction: direction,
            count: count,
            content: Vec::new(),
        }
    }
}

impl From<usize> for Array {
    fn from(value: usize) -> Self
    {
        Self {
            address: value,
            direction: Direction::InOut,
            count: 0,
            content: Vec::new(),
        }
    }
}

impl DecodeArg for Array {

    fn decode(&mut self, _pid: i32, _operation: &Box<Operation>) -> Result<(), std::io::Error> { 
       panic!("To implement"); 
    }
}

impl EncodeArg for Array {

    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
       panic!("To implement"); 
    }
}



/*
 * Represent a structure in memory
 */
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Struct {
    pub address: usize,
    pub direction: Direction,
    pub size: usize,
    pub name: String,
    pub content: Vec<u8>,
}

impl Struct {
    //pub fn new(address: usize, name: &str) -> Self {    TODO: use name during creation
    pub fn new(address: usize, direction: Direction) -> Self {
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

impl From<usize> for Struct {
    fn from(value: usize) -> Self
    {
        Self {
            address: value,
            direction: Direction::InOut,
            size: 0,
            name: String::new(),
            content: Vec::new(),
        }
    }
}

impl DecodeArg for Struct {

    fn decode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> { 
       // The best would be to know the structure for each struct and read / parse it.
       // For now read 4kB
        self.content = operation.memory.read(pid, self.address, 4096);
        Ok(())
    }

    fn print(&self) {
        println!("name: {}", self.name);
        println!("direction: {}", self.direction);
        println!("size: {:#x}", self.size);
        println!("address: {:#x}", self.address);
        println!("content: {:#x?}", self.content);
    }
}

impl EncodeArg for Struct {

    fn encode(&mut self, pid: i32, operation: &Box<Operation>) -> Result<(), std::io::Error> {
        let mem = self.content.clone();
        operation.memory.write(pid, self.address, mem);
        Ok(())
    }
}
