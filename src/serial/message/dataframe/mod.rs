use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    Request = 0x00,
    Response = 0x01,
}

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum FuncID {
    SerialAPIGetInitData = 0x02,
    ApplicationCommandHandler = 0x04,
    SerialAPIGetCapabilities = 0x07,
    SendData = 0x13,
    GetVersion = 0x15,
    GetID = 0x20,
    GetNodeProtocolInfo = 0x41,
    ApplicationUpdate = 0x49,
    AddNodeToNetwork = 0x4a,
    GetNodeInfo = 0x60,
    RemoveFailedNode = 0x61,
    IsNodeFailed = 0x62,
    ReplaceFailedNode = 0x63,
}

pub mod response;
pub use response::*;

pub mod request;
pub use request::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DataFrame {
    Response(Response),
    Request(Request),
    Unknown { type_: u8 },
}
