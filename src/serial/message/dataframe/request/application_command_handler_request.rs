use super::super::super::super::super::command_class::CommandClass;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationCommandHandlerRequest {
    pub status : u8,
    pub node_id : u8,
    pub data : CommandClass,
}

impl ApplicationCommandHandlerRequest {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.status);
        dst.put_u8(self.node_id);
        dst.put_u8(0x0);
        self.data.encode(dst);
    }

    pub fn decode(src: &mut Bytes) -> ApplicationCommandHandlerRequest {
        let status = src.get_u8();
        let node_id = src.get_u8();
        src.advance(1); // skip

        ApplicationCommandHandlerRequest { status, node_id, data: CommandClass::decode(src) }
    }
}