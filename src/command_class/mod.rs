pub mod device_locally_reset;
pub mod security2;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub enum CommandClass {
    Security2(security2::Security2),
    DeviceLocallyReset(device_locally_reset::DeviceLocallyReset),
}

impl CommandClass {

    pub fn encode(&self, dst: &mut BytesMut) {
        match self {
            CommandClass::Security2(data) => {
                dst.put_u8(security2::Security2::CLASS);
                data.encode(dst);
            },
            CommandClass::DeviceLocallyReset(data) => {
                dst.put_u8(device_locally_reset::DeviceLocallyReset::CLASS);
                data.encode(dst);
            },
        }
    }

    pub fn decode(src: &mut Bytes) -> CommandClass {
        let command = src.get_u8();
        return match command {
            security2::Security2::CLASS => {
                CommandClass::Security2(security2::Security2::decode(src))
            },
            device_locally_reset::DeviceLocallyReset::CLASS => {
                CommandClass::DeviceLocallyReset(device_locally_reset::DeviceLocallyReset::decode(src))
            },
            _ => panic!("Do not support {:?} {:?}", command, src)
        }
    }
}