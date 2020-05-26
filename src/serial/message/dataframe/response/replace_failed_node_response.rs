use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Result {
    Ok = 0x0,
    Waiting = 0x3,
    Done = 0x4,
    Failed = 0x5
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReplaceFailedNodeResponse {
    pub result : Result,
}

impl ReplaceFailedNodeResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.result.into());
        // dst.put_u8(self.result);
    }

    pub fn decode(src: &mut Bytes) -> ReplaceFailedNodeResponse {
        let result : Result = FromPrimitive::from_u8(src.get_u8()).unwrap();
        // let result = src.get_u8();
        ReplaceFailedNodeResponse { result }
    }
}