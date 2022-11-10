/*
 * Syscall data structures
 */
use std::any::Any;

use serde::{ Serialize, Deserialize };
use serde::ser::{ Serializer, SerializeStruct };

use crate::{
    tracer_engine::{
        decoder::{ Decoder, Decode, Int, Fd, Size, Offset, Flag, Prot, Signal, Address, Buffer, NullBuf, Struct },
        filtering::{ Decision },
    },
};



#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct RawSyscall {
    pub no: u64,
    pub args: Vec<u64>,
    pub retval: u64,
    pub errno: u64,
}

impl RawSyscall {
    fn new() -> Self {
        Self {
            no: 0,
            args: vec![0; 7],
            retval: 0,
            errno: 0,
        }
    }

    fn to_json(&self) -> String {
        format!("{{\"no\": {}, \"args\": {:?}, \"retval\": {}, \"errno\": {}}}", self.no, self.args, self.retval, self.errno)
    }
}


#[derive(Serialize)]
#[derive(Clone)]
pub struct Syscall {
    pub raw: RawSyscall,
    pub name: String,
    pub args: Vec<Option<Box<dyn Decode>>>,         // TODO: serde !
    pub decision: Option<Decision>,
}

impl Syscall {
    pub fn new() -> Self {
        Self {
            raw: RawSyscall::new(),
            name: String::with_capacity(25),
            //args: vec![&None; 7],
            args: Vec::from([None, None, None, None, None, None, None]),
            //decision: None,
            decision: Some(Decision::Continue), //Once the filtering implemented, put None 
        }
    }

    fn print(&self) {
        for arg in self.args.iter() {
            match arg {
                Some(a) => a.print(),
                None => break,
            }
        }
    }

    fn args_to_json(&self) -> String {
        // TODO: improve format here
        let mut s = String::new();
        s.push('[');
        for arg in self.args.iter() {
            match arg {
                Some(a) => s.push_str(&a.to_json()),
                None => break,
            }
            s.push(',');    //TODO: always add a trailing comma...
        }
        s.push(']');
        s
    }

    pub fn to_json(&self) -> String {
        // TODO: replace 0 with self.decision
        format!("{{\"raw\": {}, \"name\": \"{}\", \"args\": {}, \"decision\": {:?}}}", self.raw.to_json(), self.name, self.args_to_json(), self.decision.unwrap() as u8)
    }

}

/*
 * https://stackoverflow.com/questions/50021897/how-to-implement-serdeserialize-for-a-boxed-trait-object
 */
impl Serialize for Box<dyn Decode> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {

            let any = self.as_any();

            /*
             * TODO: reimplement with a match
             * Template:
            if any.is::<X>() {
                let obj = any.downcast_ref::<X>().unwrap();
                return // serialize
            }
             */
            if any.is::<Int>() {
                let obj = any.downcast_ref::<Int>().unwrap();
                let mut state = serializer.serialize_struct("Int", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Size>() {
                let obj = any.downcast_ref::<Size>().unwrap();
                let mut state = serializer.serialize_struct("Size", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Fd>() {
                let obj = any.downcast_ref::<Fd>().unwrap();
                let mut state = serializer.serialize_struct("Fd", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Offset>() {
                let obj = any.downcast_ref::<Offset>().unwrap();
                let mut state = serializer.serialize_struct("Offset", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Flag>() {
                let obj = any.downcast_ref::<Flag>().unwrap();
                let mut state = serializer.serialize_struct("Flag", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Prot>() {
                let obj = any.downcast_ref::<Prot>().unwrap();
                let mut state = serializer.serialize_struct("Prot", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Signal>() {
                let obj = any.downcast_ref::<Signal>().unwrap();
                let mut state = serializer.serialize_struct("Signal", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Address>() {
                let obj = any.downcast_ref::<Address>().unwrap();
                let mut state = serializer.serialize_struct("Address", 1).unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Buffer>() {
                let obj = any.downcast_ref::<Buffer>().unwrap();
                let mut state = serializer.serialize_struct("Buffer", 1).unwrap();
                state.serialize_field("address", &obj.address).unwrap();
                state.serialize_field("size", &obj.size).unwrap();
                state.serialize_field("content", &obj.content).unwrap();
                return state.end()
            }

            if any.is::<NullBuf>() {
                let obj = any.downcast_ref::<NullBuf>().unwrap();
                let mut state = serializer.serialize_struct("NullBuf", 1).unwrap();
                state.serialize_field("address", &obj.address).unwrap();
                state.serialize_field("size", &obj.size).unwrap();
                state.serialize_field("content", &obj.content).unwrap();
                return state.end()
            }

            if any.is::<Struct>() {
                let obj = any.downcast_ref::<Struct>().unwrap();
                let mut state = serializer.serialize_struct("Struct", 1).unwrap();
                state.serialize_field("address", &obj.address).unwrap();
                state.serialize_field("size", &obj.size).unwrap();
                state.serialize_field("name", &obj.name).unwrap();
                state.serialize_field("content", &obj.content).unwrap();
                return state.end()
            }

            //return S::Error
            panic!("Error during serialization downcast_ref");
    }

}

/*
 * TODO
 * see https://serde.rs/impl-deserialize.html
 * and https://serde.rs/deserialize-struct.html
impl Deserialize for Box<dyn Decode>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'de> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field { Int, Fd, Size, Offset, Flag, Prot, Signal, Address, Buffer, NullBuf, Struct };


            struct SCArguments;

            impl<'de> Visitor<'de> for SCArguments {
                type Value = Duration;
            }
    }
}
 */

