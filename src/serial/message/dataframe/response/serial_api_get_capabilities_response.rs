use bitvec::prelude::*;
use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct SerialAPIGetCapabilitiesResponse {
    pub api_version_major: u8,
    pub api_version_minor: u8,
    pub manufacturer_id: u16,
    pub product_type: u16,
    pub product_id: u16,
    pub api_mask: BitVec<Local, u8>,
}

impl SerialAPIGetCapabilitiesResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.api_version_major);
        dst.put_u8(self.api_version_minor);
        dst.put_u16(self.manufacturer_id);
        dst.put_u16(self.product_type);
        dst.put_u16(self.product_id);
        // TODO dst.put(&self.api_mask[..]);
    }

    pub fn decode(src: &mut Bytes) -> SerialAPIGetCapabilitiesResponse {
        let api_version_major = src.get_u8();
        let api_version_minor = src.get_u8();
        let manufacturer_id = src.get_u16();
        let product_type = src.get_u16();
        let product_id = src.get_u16();
        let api_mask = BitVec::from_slice(&src[0..32]);
        SerialAPIGetCapabilitiesResponse { api_version_major, api_version_minor, manufacturer_id, product_type, product_id, api_mask }
    }
}