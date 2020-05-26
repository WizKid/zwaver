use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct GetIDResponse {
    pub home_id: u32,
    pub node_id: u8,
}

impl GetIDResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u32(self.home_id);
        dst.put_u8(self.node_id);
    }

    pub fn decode(src: &mut Bytes) -> GetIDResponse {
        let home_id = src.get_u32();
        let node_id = src.get_u8();

        GetIDResponse { home_id, node_id }
    }  
}