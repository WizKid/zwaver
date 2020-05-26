use std::convert::TryInto;

use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    NodeInfoReceived = 0x84,
    NodeInfoRequestDone = 0x82,
    NodeInfoRequestFailed = 0x81,
    RoutingPending = 0x80,
    NewIDAssigned = 0x40,
    DeleteDone = 0x20,
    SucID = 0x10,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeInfoReceivedData {
    pub node_id : u8,
    pub basic : u8,
    pub generic : u8,
    pub specific : u8,
    pub supported_command_classes : Vec<u8>,
    pub controlled_command_classes : Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ApplicationUpdateRequest {
    NodeInfoReceived(NodeInfoReceivedData),
    NodeInfoRequestDone,
    NodeInfoRequestFailed,
    RoutingPending,
    NewIDAssigned,
    DeleteDone,
    SucID,
}

impl ApplicationUpdateRequest {
    pub fn encode(&self, dst: &mut BytesMut) {
        match self {
            ApplicationUpdateRequest::NodeInfoReceived(data) => {
                dst.put_u8(Type::NodeInfoReceived.into());
                dst.put_u8(data.node_id);
                dst.put_u8((3 + data.supported_command_classes.len() + 1 + data.controlled_command_classes.len()).try_into().unwrap());
                dst.put_u8(data.basic);
                dst.put_u8(data.generic);
                dst.put_u8(data.specific);
                for v in &data.supported_command_classes {
                    dst.put_u8(*v);
                }
                dst.put_u8(0xef); // Marker
                for v in &data.controlled_command_classes {
                    dst.put_u8(*v);
                }
            },
            _ => panic!("Need to implement {:?}", self)
        };
    }

    pub fn decode(src: &mut Bytes) -> ApplicationUpdateRequest {
        let type_: Type = FromPrimitive::from_u8(src.get_u8()).unwrap();
        let node_id = src.get_u8();
        match type_ {
            Type::NodeInfoReceived => {
                let len  = src.get_u8();
                let basic = src.get_u8();
                let generic = src.get_u8();
                let specific = src.get_u8();

                let mut supported_command_classes = vec![];
                let mut controlled_command_classes = vec![];
                let mut passed_marker = false;
                for _i in 0..len - 3 {
                    let command_class = src.get_u8();
                    if passed_marker {
                        controlled_command_classes.push(command_class);
                        continue;
                    }

                    if command_class == 0xef {
                        passed_marker = true;
                        continue;
                    }

                    supported_command_classes.push(command_class);
                }

                ApplicationUpdateRequest::NodeInfoReceived(
                    NodeInfoReceivedData { node_id, basic, generic, specific, supported_command_classes, controlled_command_classes },
                )
            },
            Type::NodeInfoRequestDone => {
                ApplicationUpdateRequest::NodeInfoRequestDone
            },
            Type::NodeInfoRequestFailed => {
                ApplicationUpdateRequest::NodeInfoRequestFailed
            },
            Type::RoutingPending => {
                ApplicationUpdateRequest::RoutingPending
            },
            Type::NewIDAssigned => {
                ApplicationUpdateRequest::NewIDAssigned
            },
            Type::DeleteDone => {
                ApplicationUpdateRequest::DeleteDone
            },
            Type::SucID => {
                ApplicationUpdateRequest::SucID
            },
        }
    }
}