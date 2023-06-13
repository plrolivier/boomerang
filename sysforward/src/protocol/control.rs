/*
 *
 */
use serde::{Serialize, Deserialize};


/*
    - new data channel with an address and a port to agree on (i.e. new thread)
    - inject signal
    - read / write registers
    - read / write memory
    - 
 */
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum Command {
    Ack             = 0,
    NewProcess      = 1,

    //NotifySignal    = 2,

    //ReadRegisters       = 3,
    //WriteRegister       = 4,
    //WriteRegisters      = 5,

    //ReadMemory          = 6,
    //ReadString          = 7,
    //WriteMemory         = 8,
}

/*
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum Direction {
    Request  = 0,
    Response = 1,
}
*/


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub command: Command,
    //direction: Direction,
    //length: usize,
    //payload: Vec<u8>,
    pub payload: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct AckPayload {
    pub command: Command,
    pub result: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NewProcessRequestPayload {
    pub address_ipv4: &'static str,
    pub tracer_port: u16,
    pub executor_port: u16,
}

/*
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NewProcessResponsePayload {
    pid: u32,
}
*/
