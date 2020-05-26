use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct NodeIDWithCallbackRequest {
    pub node_id : u8,
    pub callback : u8,
}

impl NodeIDWithCallbackRequest {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.node_id);
        dst.put_u8(self.callback);
    }

    pub fn decode(src: &mut Bytes) -> NodeIDWithCallbackRequest {
        let node_id = src.get_u8();
        let callback = src.get_u8();
        NodeIDWithCallbackRequest { node_id, callback }
    }
}