// mod message;

use crate::serial::message;
use crate::serial::message::dataframe;

use tokio_util::codec::{Decoder, Encoder};
use std::io;
use std::convert::TryInto;
use num_traits::FromPrimitive;
use bytes::{ BytesMut, Bytes, BufMut, Buf };

pub struct Codec;

impl Codec {
    fn calculate_checksum(buf: &[u8]) -> u8 {
        let mut checksum : u8 = 0xff;
        let len : u8 = (1 + buf.len()).try_into().unwrap();
        checksum ^= len;
        for b in buf.iter() {
            checksum ^= b;
        }
        return checksum;
    }

    fn decode_dataframe(src: &Bytes, is_host: bool) -> dataframe::DataFrame {
        println!("decode_dataframe: {:?}", src);
        return match FromPrimitive::from_u8(src[0]) {
            Some(dataframe::Type::Request) => {
                let content = match FromPrimitive::from_u8(src[1]) {
                    Some(dataframe::FuncID::SerialAPIGetInitData) => {
                        dataframe::Request::SerialAPIGetInitData
                    },
                    Some(dataframe::FuncID::SerialAPIGetCapabilities) => {
                        dataframe::Request::SerialAPIGetCapabilities
                    },
                    Some(dataframe::FuncID::GetVersion) => {
                        dataframe::Request::GetVersion
                    },
                    Some(dataframe::FuncID::GetID) => {
                        dataframe::Request::GetID
                    },
                    Some(dataframe::FuncID::AddNodeToNetwork) => {
                        if is_host {
                            dataframe::Request::AddNodeToNetworkHost(
                                dataframe::AddNodeToNetworkRequestHost::decode(&mut src.slice(2..))
                            )
                        } else {
                            dataframe::Request::AddNodeToNetworkChip(
                                dataframe::AddNodeToNetworkRequestChip::decode(&mut src.slice(2..))
                            )
                        }
                    },
                    Some(dataframe::FuncID::GetNodeProtocolInfo) => {
                        dataframe::Request::GetNodeProtocolInfo(
                            dataframe::NodeIDRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::GetNodeInfo) => {
                        dataframe::Request::GetNodeInfo(
                            dataframe::NodeIDRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::ApplicationUpdate) => {
                        dataframe::Request::ApplicationUpdate(
                            dataframe::ApplicationUpdateRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::RemoveFailedNode) => {
                        dataframe::Request::RemoveFailedNode(
                            dataframe::NodeIDWithCallbackRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::IsNodeFailed) => {
                        dataframe::Request::IsNodeFailed(
                            dataframe::NodeIDRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::ReplaceFailedNode) => {
                        dataframe::Request::ReplaceFailedNode(
                            dataframe::NodeIDRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::ApplicationCommandHandler) => {
                        dataframe::Request::ApplicationCommandHandler(
                            dataframe::ApplicationCommandHandlerRequest::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::SendData) => {
                        if is_host {
                            dataframe::Request::SendDataHost(
                                dataframe::SendDataRequestHost::decode(&mut src.slice(2..))
                            )
                        } else {
                            dataframe::Request::SendDataChip(
                                dataframe::SendDataRequestChip::decode(&mut src.slice(2..))
                            )
                        }
                    },
                    _ => dataframe::Request::Unknown { func_id: src[1], data: src.slice(2..) }
                };

                dataframe::DataFrame::Request(content)
            },
            Some(dataframe::Type::Response) => {
                let content = match FromPrimitive::from_u8(src[1]) {
                    Some(dataframe::FuncID::SerialAPIGetInitData) => {
                        dataframe::Response::SerialAPIGetInitData(
                            dataframe::SerialAPIGetInitDataResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::SerialAPIGetCapabilities) => {
                        dataframe::Response::SerialAPIGetCapabilities(
                            dataframe::SerialAPIGetCapabilitiesResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::GetVersion) => {
                        dataframe::Response::GetVersion(
                            dataframe::GetVersionResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::GetID) => {
                        dataframe::Response::GetID(
                            dataframe::GetIDResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::GetNodeProtocolInfo) => {
                        dataframe::Response::GetNodeProtocolInfo(
                            dataframe::GetNodeProtocolInfoResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::GetNodeInfo) => {
                        dataframe::Response::GetNodeInfo(
                            dataframe::BoolResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::IsNodeFailed) => {
                        dataframe::Response::IsNodeFailed(
                            dataframe::BoolResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::RemoveFailedNode) => {
                        dataframe::Response::RemoveFailedNode(
                            dataframe::RemoveFailedNodeResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::ReplaceFailedNode) => {
                        dataframe::Response::ReplaceFailedNode(
                            dataframe::ReplaceFailedNodeResponse::decode(&mut src.slice(2..))
                        )
                    },
                    Some(dataframe::FuncID::SendData) => {
                        dataframe::Response::SendData(
                            dataframe::BoolResponse::decode(&mut src.slice(2..))
                        )
                    },
                    _ => dataframe::Response::Unknown { func_id: src[1], data: src.slice(2..) }
                };
                dataframe::DataFrame::Response(content)
            },
            None => dataframe::DataFrame::Unknown { type_: src[0] },
        }
    }

    pub fn read_string(buf: &mut Bytes) -> String {
        let mut i = 0;
        for b in buf.iter() {
            if *b == 0x0 {
                break;
            }
            i += 1;
        }
        let ret = String::from_utf8(buf.slice(0..i).to_vec()).unwrap();
        buf.advance(i+1);
        ret
    }

    fn encode_dataframe(data_frame: &dataframe::DataFrame, dst: &mut BytesMut) -> Result<(), io::Error> {
        dst.reserve(256);
        let mut data_frame_dst = dst.split_off(dst.len() + 1);

        match data_frame {
            dataframe::DataFrame::Request(request) => {
                data_frame_dst.put_u8(dataframe::Type::Request.into());
                match request {
                    dataframe::Request::SerialAPIGetInitData => {
                        data_frame_dst.put_u8(dataframe::FuncID::SerialAPIGetInitData.into());
                    },
                    dataframe::Request::SerialAPIGetCapabilities => {
                        data_frame_dst.put_u8(dataframe::FuncID::SerialAPIGetCapabilities.into());
                    },
                    dataframe::Request::GetVersion => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetVersion.into());
                    },
                    dataframe::Request::GetID => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetID.into());
                    },
                    dataframe::Request::AddNodeToNetworkHost(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::AddNodeToNetwork.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::AddNodeToNetworkChip(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::AddNodeToNetwork.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::GetNodeProtocolInfo(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetNodeProtocolInfo.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::GetNodeInfo(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetNodeInfo.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::ApplicationUpdate(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::ApplicationUpdate.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::IsNodeFailed(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::IsNodeFailed.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::RemoveFailedNode(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::RemoveFailedNode.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::ReplaceFailedNode(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::ReplaceFailedNode.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::ApplicationCommandHandler(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::ApplicationCommandHandler.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::SendDataHost(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::SendData.into());
                        data.encode(&mut data_frame_dst);
                    },
                    dataframe::Request::SendDataChip(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::SendData.into());
                        data.encode(&mut data_frame_dst);
                    },
                    _ => {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Can't send request {:?}", request)));
                    }
                }
            }
            dataframe::DataFrame::Response(response) => {
                data_frame_dst.put_u8(dataframe::Type::Response.into());
                match response {
                    dataframe::Response::SerialAPIGetInitData(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::SerialAPIGetInitData.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::SerialAPIGetCapabilities(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::SerialAPIGetCapabilities.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::GetVersion(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetVersion.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::GetID(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetID.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::GetNodeProtocolInfo(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetNodeProtocolInfo.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::GetNodeInfo(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::GetNodeInfo.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::RemoveFailedNode(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::RemoveFailedNode.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::IsNodeFailed(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::IsNodeFailed.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::ReplaceFailedNode(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::ReplaceFailedNode.into());
                        data.encode(&mut data_frame_dst)
                    },
                    dataframe::Response::SendData(data) => {
                        data_frame_dst.put_u8(dataframe::FuncID::SendData.into());
                        data.encode(&mut data_frame_dst)
                    },
                    _ => {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Can't send response {:?}", response)));
                    }
                }
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Can't send data_frame {:?}", data_frame)))
        }

        dst.put_u8((data_frame_dst.len() + 1).try_into().unwrap());
        let checksum = Codec::calculate_checksum(&data_frame_dst[0..]);
        dst.unsplit(data_frame_dst);
        dst.put_u8(checksum);

        Ok(())
    }
}

impl Decoder for Codec {
    type Item = message::Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None)
        }
        return match FromPrimitive::from_u8(src[0]) {
            Some(message::Type::ACK) => {
                src.advance(1);
                Ok(Some(message::Message::Ack))
            },
            Some(message::Type::NAK) => {
                src.advance(1);
                Ok(Some(message::Message::Nak))
            },
            Some(message::Type::CAN) => {
                src.advance(1);
                Ok(Some(message::Message::Can))
            },
            Some(message::Type::DATAFRAME) => {
                println!("DataFrame: {:?}", src);
                if src.len() < 2 {
                    // Haven't gotten the length yet so ask for more data
                    return Ok(None);
                }
                let len = usize::from(src[1]);
                if len < 3 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("size {} < 3", len)));
                }

                if src.len() < len + 2 {
                    // Haven't gotten enough to parse the whole message so ask for more data
                    return Ok(None);
                }
                src.advance(2);
                let buf = Bytes::from(src.split_to(len - 1));

                let checksum = src[0];
                src.advance(1);

                let calculated_checksum = Codec::calculate_checksum(&buf);
                if  calculated_checksum != checksum {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid checksum {} != {}, {:?}", calculated_checksum, checksum, buf)));
                }

                Ok(Some(message::Message::DataFrame(Codec::decode_dataframe(&buf, false))))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown message type: {:?}", src[0]))),
        }
    }
}

impl Encoder<message::Message> for Codec {
    type Error = io::Error;

    fn encode(&mut self, item: message::Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            message::Message::Ack => dst.put_u8(message::Type::ACK.into()),
            message::Message::Nak => dst.put_u8(message::Type::NAK.into()),
            message::Message::Can => dst.put_u8(message::Type::CAN.into()),
            message::Message::DataFrame(data_frame) => {
                dst.put_u8(message::Type::DATAFRAME.into());
                let res = Codec::encode_dataframe(&data_frame, dst);
                if let  Err(e) = res {
                    return Err(e);
                }
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tokio::{stream::StreamExt};
    use tokio_util::codec::{FramedParts, Framed};
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::io::AsyncRead;
    use tokio_test::assert_ok;

    struct DontReadIntoThis;

    impl io::Read for DontReadIntoThis {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Read into something you weren't supposed to.",
            ))
        }
    }

    impl AsyncRead for DontReadIntoThis {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn test_encoder() {
        let mut output = BytesMut::new();
        let mut codec = Codec;

        codec.encode(message::Message::Ack, &mut output).expect("Invalid encoding sequence");
        codec.encode(message::Message::Nak, &mut output).expect("Invalid encoding sequence");
        codec.encode(message::Message::Can, &mut output).expect("Invalid encoding sequence");
        {
            let msg = message::Message::DataFrame(
                dataframe::DataFrame::Request(
                    dataframe::Request::GetVersion,
                ),
            );
            codec.encode(msg, &mut output).expect("Invalid encoding sequence");
        }
        {
            let msg = message::Message::DataFrame(
                dataframe::DataFrame::Response(
                    dataframe::Response::GetVersion(
                        dataframe::GetVersionResponse { library_version: String::from("a"), controller_type: 0x01 },
                    ),
                ),
            );
            codec.encode(msg, &mut output).expect("Invalid encoding sequence");
        }

        assert_eq!(
            output,
            // The output should have the following bytes in the buffer
            BytesMut::from(&[
                0x06,
                0x15,
                0x18,
                0x01, 0x3, 0x0, 0x15, 0xe9,
                0x01, 0x6, 0x01, 0x15, 0x61, 0x0, 0x1, 0x8d
            ][..]),
        );
    }

    #[tokio::test]
    async fn test_decoder() {
        let mut parts = FramedParts::new(DontReadIntoThis, Codec);
        parts.read_buf = BytesMut::from(&[
            0x06,
            0x15,
            0x18,
            0x01, 0x3, 0x0, 0x15, 0xe9,
            0x01, 0x6, 0x01, 0x15, 0x61, 0x0, 0x1, 0x8d
        ][..]);

        let mut framed = Framed::from_parts(parts);

        {
            let message = assert_ok!(framed.next().await.unwrap());
            assert_eq!(message, message::Message::Ack);
        }
        {
            let message = assert_ok!(framed.next().await.unwrap());
            assert_eq!(message, message::Message::Nak);
        }
        {
            let message = assert_ok!(framed.next().await.unwrap());
            assert_eq!(message, message::Message::Can);
        }
        {
            let message = assert_ok!(framed.next().await.unwrap());
            assert_eq!(
                message,
                message::Message::DataFrame(
                    dataframe::DataFrame::Request(
                        dataframe::Request::GetVersion,
                    ),
                )
            );
        }
        {
            let message = assert_ok!(framed.next().await.unwrap());
            assert_eq!(
                message,
                message::Message::DataFrame(
                    dataframe::DataFrame::Response(
                        dataframe::Response::GetVersion(
                            dataframe::GetVersionResponse { library_version: String::from("a"), controller_type: 0x01 },
                        ),
                    ),
                )
            );
        }
    }
}
