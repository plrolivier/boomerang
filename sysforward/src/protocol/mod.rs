/*
use prost::Message;

//pub use crate::proto::messages::*;
pub mod psf {
    include!(concat!(env!("OUT_DIR"), "/proto.sysfwd.rs"));
}



// Helper function to serialize a message into a byte array
pub fn serialize_message<M>(message: &M) -> Vec<u8>
where
    M: Message,
{
    let mut buf = Vec::new();
    message.encode(&mut buf).unwrap();
    buf
}

// Helper function to parse a message from a byte array
pub fn parse_message<M>(bytes: &[u8]) -> Result<M, prost::DecodeError>
where
    M: Message + Default,
{
    M::decode(bytes)
}



fn create_ack_msg(code: u32, error_message: String) -> psf::Message {
    let mut message = psf::Message::default();
    message.msg = Some(psf::message::Msg::Ack(psf::Ack {
        code: code as i32,
        error: Some(error_message),
    }));
    message
}

fn create_notify_new_process_msg(pid: u32) -> psf::Message {
    let mut message = psf::Message::default();
    message.msg = Some(psf::message::Msg::NewProcess(psf::NotifyNewProcess {
        pid: pid,
    }));
    message
}
*/
