use super::super::super::super::codec::Codec;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct GetVersionResponse {
    pub library_version: String,
    pub controller_type: u8,
}

impl GetVersionResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_slice(self.library_version.as_bytes());
        dst.put_u8(0x0);
        dst.put_u8(self.controller_type);
    }

    pub fn decode(src: &mut Bytes) -> GetVersionResponse {
        let library_version = Codec::read_string(src);
        let controller_type = src.get_u8();

        GetVersionResponse { library_version, controller_type}
    }  
}