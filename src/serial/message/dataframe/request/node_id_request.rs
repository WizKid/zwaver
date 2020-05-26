use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct NodeIDRequest {
    pub node_id : u8,
}

impl NodeIDRequest {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.node_id);
    }

    pub fn decode(src: &mut Bytes) -> NodeIDRequest {
        let node_id = src.get_u8(); // skip
        NodeIDRequest { node_id }
    }
}