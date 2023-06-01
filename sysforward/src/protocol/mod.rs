/*
 *
 */

pub mod message;
pub mod dispatcher;
pub mod worker;

mod udp;
//mod tcp;



/* Static variable to change */
static IP_ADDRESS: &str = "127.0.0.1";
static TRACER_PORT: u16 = 31000;
static EXECUTOR_PORT: u16 = 31001;


pub trait Peer {
    fn send(&self, data: &[u8]) -> Result<(), std::io::Error>;
    fn receive(&self, buffer: &mut [u8]) -> Result<usize, std::io::Error>;
}
