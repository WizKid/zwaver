use num_derive::FromPrimitive;    
use num_enum::IntoPrimitive;

pub mod dataframe;

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Type {
    ACK = 0x6,
    NAK = 0x15,
    CAN = 0x18,
    DATAFRAME = 0x01,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Ack,
    Nak,
    Can,
    DataFrame(dataframe::DataFrame),
}