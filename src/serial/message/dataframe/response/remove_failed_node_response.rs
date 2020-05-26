use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Result {
    Ok = 0x0,
    NotPrimaryController = 0x1 << 1,
    NoCallbackFunction = 0x1 << 2,
    NodeNotFound = 0x1 << 3,
    RemoveProcessBusy = 0x1 << 4,
    RemoveFailed = 0x1 << 5,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveFailedNodeResponse {
    pub result : Result,
}

impl RemoveFailedNodeResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.result.into());
    }

    pub fn decode(src: &mut Bytes) -> RemoveFailedNodeResponse {
        let result : Result = FromPrimitive::from_u8(src.get_u8()).unwrap();
        RemoveFailedNodeResponse { result }
    }
}