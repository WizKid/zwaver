use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Status {
    LearnReady = 0x1,
    NodeFound = 0x2,
    AddingSlave = 0x3,
    AddingController = 0x4,
    ProtocolDone = 0x5,
    Done = 0x6,
    Failed = 0x7,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolDoneData {
    pub node_id: u8,
}

impl ProtocolDoneData {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.node_id);
    }

    pub fn decode(src: &mut Bytes) -> ProtocolDoneData {
        let node_id = src.get_u8();
        ProtocolDoneData { node_id }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddNodeToNetworkRequestChip {
    LearnReady,
    NodeFound,
    AddingController,
    AddingSlave,
    ProtocolDone(ProtocolDoneData),
    Done,
    Failed,
}

impl AddNodeToNetworkRequestChip {
    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(0x0); // Unknown
        match self {
            AddNodeToNetworkRequestChip::LearnReady => {
                dst.put_u8(Status::LearnReady.into());
            },
            AddNodeToNetworkRequestChip::NodeFound => {
                dst.put_u8(Status::NodeFound.into());
            },
            AddNodeToNetworkRequestChip::AddingController => {
                dst.put_u8(Status::AddingController.into());
            },
            AddNodeToNetworkRequestChip::AddingSlave => {
                dst.put_u8(Status::AddingSlave.into());
            },
            AddNodeToNetworkRequestChip::ProtocolDone( data ) => {
                dst.put_u8(Status::ProtocolDone.into());
                data.encode(dst);
            },
            AddNodeToNetworkRequestChip::Done => {
                dst.put_u8(Status::Done.into());
            },
            AddNodeToNetworkRequestChip::Failed => {
                dst.put_u8(Status::Failed.into());
            },
        }
    }

    pub fn decode(src: &mut Bytes) -> AddNodeToNetworkRequestChip {
        src.advance(1); // skip
        let status : Status = FromPrimitive::from_u8(src.get_u8()).unwrap();
        match status {
            Status::LearnReady => AddNodeToNetworkRequestChip::LearnReady,
            Status::NodeFound => AddNodeToNetworkRequestChip::NodeFound,
            Status::AddingController => AddNodeToNetworkRequestChip::AddingController,
            Status::AddingSlave => AddNodeToNetworkRequestChip::AddingSlave,
            Status::ProtocolDone => AddNodeToNetworkRequestChip::ProtocolDone(ProtocolDoneData::decode(src)),
            Status::Done => AddNodeToNetworkRequestChip::Done,
            Status::Failed => AddNodeToNetworkRequestChip::Failed,
        }
    }
}