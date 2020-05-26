use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct BoolResponse {
    pub result : bool,
}

impl BoolResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        let byte = if self.result {
            0x1
        } else {
            0x0
        };
        dst.put_u8(byte);
    }

    pub fn decode(src: &mut Bytes) -> BoolResponse {
        let result : bool = src.get_u8() != 0x0;
        BoolResponse { result }
    }
}