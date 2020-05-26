use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    Notification = 0x01,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeviceLocallyReset {
    Notification,
}

impl DeviceLocallyReset {

    pub const CLASS : u8 = 0x5a;

    pub fn encode(&self, dst: &mut BytesMut) {
        match self {
            DeviceLocallyReset::Notification => {
                dst.put_u8(Command::Notification.into());
            },
            _ => panic!("Not supported {:?}", self)
        }
    }

    pub fn decode(src: &mut Bytes) -> DeviceLocallyReset {
        let command: Option<Command> = FromPrimitive::from_u8(src.get_u8());
        return match command {
            Some(Command::Notification) => {
                DeviceLocallyReset::Notification
            },
            _ => panic!("Do not support {:?}", command)
        }
    }
}