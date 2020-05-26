use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(Debug, PartialEq, Clone)]
pub struct GetNodeProtocolInfoResponse {
    pub data: [u8; 3],
    pub basic: u8,
    pub generic: u8,
    pub specific: u8,
}

impl GetNodeProtocolInfoResponse {
    pub fn encode(&self, dst: &mut BytesMut) {
        // dst.put(Buf::from(self.data));
        dst.put_u8(self.data[0]);
        dst.put_u8(self.data[1]);
        dst.put_u8(self.data[2]);
        dst.put_u8(self.basic);
        dst.put_u8(self.generic);
        dst.put_u8(self.specific);
    }

    pub fn decode(src: &mut Bytes) -> GetNodeProtocolInfoResponse {
        let mut data: [u8; 3] = [0, 0, 0];
        data[0] = src.get_u8();
        data[1] = src.get_u8();
        data[2] = src.get_u8();
        let basic = src.get_u8();
        let generic = src.get_u8();
        let specific = src.get_u8();
        GetNodeProtocolInfoResponse { data, basic, generic, specific }
    }
}