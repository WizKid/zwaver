use std::convert::TryInto;

use super::super::super::super::super::command_class::CommandClass;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct SendDataRequestHost {
    pub node_id : u8,
    pub data : CommandClass,
    pub options : u8,
    pub callback_id: u8,
}

impl SendDataRequestHost {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.node_id);

        let mut cc_dst = dst.split_off(dst.len() + 1);
        self.data.encode(&mut cc_dst);

        dst.put_u8((cc_dst.len()).try_into().unwrap());
        dst.unsplit(cc_dst);
        dst.put_u8(self.options);
        dst.put_u8(self.callback_id);
    }

    pub fn decode(src: &mut Bytes) -> SendDataRequestHost {
        let node_id = src.get_u8();
        let len = src.get_u8();
        let mut cc_src = src.split_to(len as usize);
        let data = CommandClass::decode(&mut cc_src);
        let options = src.get_u8();
        let callback_id = src.get_u8();

        return SendDataRequestHost { node_id, data, options, callback_id }
    }
}