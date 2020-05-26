use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    Any = 0x01,
    Controller = 0x02,
    Slave = 0x03,
    Existing = 0x04,
    Stop = 0x05,
    StopFailed = 0x06,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddNodeToNetworkRequestHost {
    pub type_ : Type,
    pub high_power : bool,
    pub network_wide : bool,
}

impl AddNodeToNetworkRequestHost {
    pub fn encode(&self, dst: &mut BytesMut) {
        let mut byte: u8 = self.type_.into();
        if self.high_power {
            byte |= 0x80;
        }
        if self.network_wide {
            byte |= 0x40;
        }
        dst.put_u8(byte);
    }

    pub fn decode(src: &mut Bytes) -> AddNodeToNetworkRequestHost {
        src.advance(1); // skip
        let byte = src.get_u8();

        let type_ : Type = FromPrimitive::from_u8(byte & 0xf).unwrap();
        let high_power = byte & 0x80 != 0;
        let network_wide = byte & 0x40 != 0;

        AddNodeToNetworkRequestHost { type_, high_power, network_wide }
    }
}