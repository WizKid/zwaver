use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct SendDataRequestChip {
    pub callback_id: u8,
    pub status: u8,
}

impl SendDataRequestChip {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.callback_id);
        dst.put_u8(self.status);
    }

    pub fn decode(src: &mut Bytes) -> SendDataRequestChip {
        let callback_id = src.get_u8();
        let status = src.get_u8();

        // TODO Metrics

        return SendDataRequestChip { callback_id, status }
    }
}