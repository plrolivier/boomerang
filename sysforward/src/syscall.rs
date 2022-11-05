/*
 * Decoding for syscall arguments.
 */
pub mod ArgumentType {
    /*
    use nix::libc::c_int;
    use nix::sys::signal::Signal as NixSignal;
    */
    use nix::unistd::Pid;

    use crate::tracer_engine::tracer::Operation;



    pub trait Decode {
        fn decode(&mut self, pid: Pid, operation: &Box<dyn Operation>) { }
        fn to_json(&self) -> String { format!("{{\"type\":none,\"value\":0}}") }
        fn print(&self) { }
    }

    /* Direct value */
    pub struct Int { value: u64 }
    impl Int {
        pub fn new(value: u64) -> Self {
            Self { value: value }
        }
    }
    impl Decode for Int { 
        fn to_json(&self) -> String {
            format!("{{\"type\":\"integer\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("value: {:#x}", self.value);
        }
    }

    pub struct Fd { value: u16 }
    impl Fd {
        pub fn new(value: u64) -> Self {
            Self { value: value as u16 }
        }
    }
    impl Decode for Fd {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"fd\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("fd: {:#x}", self.value);
        }
    }

    pub struct Size { value: u64 }
    impl Size {
        pub fn new(value: u64) -> Self {
            Self { value: value }
        }
    }
    impl Decode for Size {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"size\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("size: {:#x}", self.value);
        }
    }

    pub struct Offset { value: u64 }
    impl Offset {
        pub fn new(value: u64) -> Self {
            Self { value: value }
        }
    }
    impl Decode for Offset {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"offset\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("offset: {:#x}", self.value);
        }
    }

    pub struct Flag { value: u8 }
    impl Flag {
        pub fn new(value: u64) -> Self {
            Self { value: value as u8 }
        }
    }
    impl Decode for Flag {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"flag\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("flags: {:#x}", self.value);
        }
    }

    pub struct Prot { value: u8 }
    impl Prot {
        pub fn new(value: u64) -> Self {
            Self { value: value as u8 }
        }
    }
    impl Decode for Prot {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"prot\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("prot: {:#x}", self.value);
        }
    }

    pub struct Signal {
        value: u8,
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
        fn to_json(&self) -> String {
            format!("{{\"type\":\"signal\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("signo: {:#x}", self.value);
        }
    }


    /* Pointers */
    pub struct Address {
        value: u64
    }
    impl Address {
        pub fn new(value: u64) -> Self {
            Self { value: value }
        }
    }
    impl Decode for Address {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"address\",\"value\":{}}}", self.value)
        }

        fn print(&self) {
            println!("address: {:#x}", self.value);
        }
    }

    pub struct Buffer {
        address: u64,
        size: u64,
        content: Vec<u32>,
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
        fn decode(&mut self, pid: Pid, operation: &Box<dyn Operation>) { 
            self.content = operation.read_memory(pid, self.address, self.size);
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

    pub struct NullBuf {
        address: u64,
        size: u64,
        content: Vec<u32>,
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
        fn decode(&mut self, pid: Pid, operation: &Box<dyn Operation>) { 
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


    pub struct Struct {
        address: u64,
        name: String,
        content: Vec<u32>,
    }
    impl Struct {
        pub fn new(address: u64, name: &str) -> Self {
            Self { 
                address: address,
                name: name.to_string(),
                content: Vec::new(),  // TODO: initialize with a default size?
            }
        }
    }
    impl Decode for Struct {
        fn to_json(&self) -> String {
            format!("{{\"type\":\"buffer\",\"value\":{{\"address\":{},\"name\":{},\"content\":{:?}}}}}", self.address, self.name, self.content)
        }

        fn print(&self) {
            println!("name: {}", self.name);
            println!("address: {:#x}", self.address);
            println!("content: {:#x?}", self.content);
        }
    }

}

