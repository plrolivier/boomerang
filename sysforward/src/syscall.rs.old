/*
 * Syscall data structures
 */
use std::any::Any;

use serde::{ Serialize, Deserialize };
use serde::ser::{ Serializer, SerializeStruct };
use serde::de::{ self, Deserialize, Deserializer, Visitor, MapAccess };

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
    pub entry_decoded: bool,
    pub name: String,
    pub args: Vec<Option<Box<dyn Decode>>>,         // TODO: serde !
    pub decision: Option<Decision>,
}

impl Syscall {
    pub fn new() -> Self {
        Self {
            raw: RawSyscall::new(),
            entry_decoded: false,
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
            // TODO: add a field name indicating the struct type
            if any.is::<Int>() {
                let obj = any.downcast_ref::<Int>().unwrap();
                let mut state = serializer.serialize_struct("Int", 1).unwrap();
                state.serialize_field("name", "integer").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Size>() {
                let obj = any.downcast_ref::<Size>().unwrap();
                let mut state = serializer.serialize_struct("Size", 1).unwrap();
                state.serialize_field("name", "size").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Fd>() {
                let obj = any.downcast_ref::<Fd>().unwrap();
                let mut state = serializer.serialize_struct("Fd", 1).unwrap();
                state.serialize_field("name", "fd").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Offset>() {
                let obj = any.downcast_ref::<Offset>().unwrap();
                let mut state = serializer.serialize_struct("Offset", 1).unwrap();
                state.serialize_field("name", "offset").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Flag>() {
                let obj = any.downcast_ref::<Flag>().unwrap();
                let mut state = serializer.serialize_struct("Flag", 1).unwrap();
                state.serialize_field("name", "flag").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Prot>() {
                let obj = any.downcast_ref::<Prot>().unwrap();
                let mut state = serializer.serialize_struct("Prot", 1).unwrap();
                state.serialize_field("name", "prot").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Signal>() {
                let obj = any.downcast_ref::<Signal>().unwrap();
                let mut state = serializer.serialize_struct("Signal", 1).unwrap();
                state.serialize_field("name", "signal").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Address>() {
                let obj = any.downcast_ref::<Address>().unwrap();
                let mut state = serializer.serialize_struct("Address", 1).unwrap();
                state.serialize_field("name", "address").unwrap();
                state.serialize_field("value", &obj.value).unwrap();
                return state.end()
            }

            if any.is::<Buffer>() {
                let obj = any.downcast_ref::<Buffer>().unwrap();
                let mut state = serializer.serialize_struct("Buffer", 1).unwrap();
                state.serialize_field("name", "buffer").unwrap();
                state.serialize_field("address", &obj.address).unwrap();
                state.serialize_field("size", &obj.size).unwrap();
                state.serialize_field("content", &obj.content).unwrap();
                return state.end()
            }

            if any.is::<NullBuf>() {
                let obj = any.downcast_ref::<NullBuf>().unwrap();
                let mut state = serializer.serialize_struct("NullBuf", 1).unwrap();
                state.serialize_field("name", "nullbuf").unwrap();
                state.serialize_field("address", &obj.address).unwrap();
                state.serialize_field("size", &obj.size).unwrap();
                state.serialize_field("content", &obj.content).unwrap();
                return state.end()
            }

            if any.is::<Struct>() {
                let obj = any.downcast_ref::<Struct>().unwrap();
                let mut state = serializer.serialize_struct("Struct", 1).unwrap();
                state.serialize_field("name", "struct").unwrap();
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
 */
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

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct SCArguments")
                }

                fn visit_map<V>(self, mut map: V) -> Result<Box<dyn Decode>, V::Error>
                    where V: MapAccess<'de> {
                        let mut integer = None;
                        let mut fd = None;
                        let mut size = None;
                        let mut offset = None;
                        let mut flag = None;
                        let mut prot = None;
                        let mut signal = None;
                        let mut address = None;
                        let mut buffer = None;
                        let mut nullbuf = None;
                        let mut structure = None;

                        //The first entry is considered to be name with the structure name
                        while let Some(key) = map.next_key().unwrap() {
                            match key {
                                Field::Int => {
                                    if integer.is_some() {
                                        return Err(de::Error::duplicate_field("integer"));
                                    }
                                    integer = Some(map.next_value().unwrap());
                                }
                                Field::Fd => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("fd"));
                                    }
                                    fd = Some(map.next_value().unwrap());
                                }
                                Field::Size => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("size"));
                                    }
                                    size = Some(map.next_value().unwrap());
                                }
                                Field::Offset => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("offset"));
                                    }
                                    offset = Some(map.next_value().unwrap());
                                }
                                Field::Flag => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("Flag"));
                                    }
                                    flag = Some(map.next_value().unwrap());
                                }
                                Field::Prot => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("prot"));
                                    }
                                    prot = Some(map.next_value().unwrap());
                                }
                                Field::Signal => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("signal"));
                                    }
                                    signal = Some(map.next_value().unwrap());
                                }
                                Field::Address => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("address"));
                                    }
                                    address = Some(map.next_value().unwrap());
                                }
                                Field::Buffer => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("buffer"));
                                    }
                                    buffer = Some(map.next_value().unwrap());
                                }
                                Field::NullBuf => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("nullbuf"));
                                    }
                                    nullbuf = Some(map.next_value().unwrap());
                                }
                                Field::Struct => {
                                    if .is_some() {
                                        return Err(de::Error::duplicate_field("structure"));
                                    }
                                    structure = Some(map.next_value().unwrap());
                                }
                            }
                        }
                        let integer = integer.ok_or_else(|| de::Error::missing_field("integer")).unwrap();
                        let fd = fd.ok_or_else(|| de::Error::missing_field("fd")).unwrap();
                        let size = size.ok_or_else(|| de::Error::missing_field("size")).unwrap();
                        let offset = offset.ok_or_else(|| de::Error::missing_field("offset")).unwrap();
                        let flag = flag.ok_or_else(|| de::Error::missing_field("flag")).unwrap();
                        let prot = prot.ok_or_else(|| de::Error::missing_field("prot")).unwrap();
                        let signal = signal.ok_or_else(|| de::Error::missing_field("signal")).unwrap();
                        let address = address.ok_or_else(|| de::Error::missing_field("address")).unwrap();
                        let buffer = buffer.ok_or_else(|| de::Error::missing_field("buffer")).unwrap();
                        let nullbuf = nullbuf.ok_or_else(|| de::Error::missing_field("nullbuf")).unwrap();
                        let structure = structure.ok_or_else(|| de::Error::missing_field("struct")).unwrap();
                        Ok()
                }
            }
    }
}

