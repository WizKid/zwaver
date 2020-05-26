use bitvec::prelude::*;
use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct SerialAPIGetInitDataResponse {
    pub init_version: u8,
    pub init_caps: u8,
    pub nodes: BitVec<Local, u8>,
}

impl SerialAPIGetInitDataResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.init_version);
        dst.put_u8(self.init_caps);
        dst.put_u8(29);
        // TODO dst.put(&self.nodes[..]);
    }

    pub fn decode(src: &mut Bytes) -> SerialAPIGetInitDataResponse {
        let init_version = src.get_u8();
        let init_caps = src.get_u8();
        let _len = src.get_u8();
        // _len should always be 29
        let nodes = BitVec::from_slice(&src[0..29]);
        SerialAPIGetInitDataResponse { init_version, init_caps, nodes }
    }
}