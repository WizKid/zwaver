use num_derive::FromPrimitive;
use num_enum::IntoPrimitive;
use num_traits::FromPrimitive;

use bytes::{ Bytes, Buf, BytesMut, BufMut };

#[derive(FromPrimitive, IntoPrimitive, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    NonceGet = 0x01,
    NonceReport = 0x02,
    MessageEncapsulation = 0x03,
    KexGet = 0x04,
    KexReport = 0x05,
    KexSet = 0x06,
    KexFail = 0x07,
    PublicKeyReport = 0x08,
    NetworkKeyGet = 0x09,
    NetworkKeyReport = 0x0a,
    NetworkKeyVerify = 0x0b,
    TransferEnd = 0x0c,
    CommandsSupportedGet = 0x0d,
    CommandsSupportedReport = 0x0e,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KexData {
    pub request_csa : bool,
    pub echo : bool,
    pub kex_schemes : u8,
    pub ecdh_profiles : u8,
    pub keys : u8,
}

impl KexData {

    pub fn encode(&self, dst: &mut BytesMut) {
        let mut byte = 0x0;
        if self.request_csa {
            byte |= 0x2;
        }
        if self.echo {
            byte |= 0x1;
        }
        dst.put_u8(byte);
        dst.put_u8(self.kex_schemes);
        dst.put_u8(self.ecdh_profiles);
        dst.put_u8(self.keys);
    }

    pub fn decode(src: &mut Bytes) -> KexData {
        let byte = src.get_u8();
        let request_csa = byte & 0x2 != 0;
        let echo = byte & 0x1 != 0;
        let kex_schemes = src.get_u8();
        let ecdh_profiles = src.get_u8();
        let keys = src.get_u8();
        KexData { request_csa, echo, kex_schemes, ecdh_profiles, keys }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KexFailData {
    pub type_ : u8,
}

impl KexFailData {

    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.type_);
    }

    pub fn decode(src: &mut Bytes) -> KexFailData {
        let type_ = src.get_u8();
        KexFailData { type_ }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PublicKeyReportData {
    pub including_node : bool,
    pub key: [u8; 32],
}

impl PublicKeyReportData {

    pub fn encode(&self, dst: &mut BytesMut) {
        let mut byte = 0x0;
        if self.including_node {
            byte |= 0x1;
        }
        dst.put_u8(byte);
        for i in 0..32 {
            dst.put_u8(self.key[i]);
        }
    }

    pub fn decode(src: &mut Bytes) -> PublicKeyReportData {
        let including_node = src.get_u8() & 0x1 != 0;
        let mut key: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..32 {
            key[i] = src.get_u8();
        }
        PublicKeyReportData { including_node, key }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MessageEncapsulationData {
    pub sequence_number : u8,
    pub encrypted_extension : bool,
    pub extension : bool,
    pub extensions : Vec<Bytes>,
}

impl MessageEncapsulationData {

    pub fn encode(&self, dst: &mut BytesMut) {

    }

    pub fn decode(src: &mut Bytes) -> MessageEncapsulationData {
        MessageEncapsulationData { sequence_number: 0, encrypted_extension: false, extension: false, extensions: vec![] }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NonceGetData {
    pub sequence_number : u8,
}

impl NonceGetData {

    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.sequence_number);
    }

    pub fn decode(src: &mut Bytes) -> NonceGetData {
        let sequence_number = src.get_u8();
        NonceGetData { sequence_number }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NonceReportData {
    pub sequence_number : u8,
    pub mos : bool,
    pub sos : bool,
    pub entropy_input : Option<Bytes>,
}

impl NonceReportData {

    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(self.sequence_number);
        let mut byte = 0x0;
        if self.mos {
            byte |= 0x2;
        }
        if self.sos {
            byte |= 0x1;
        }
        dst.put_u8(byte);
        if let Some(bytes) = self.entropy_input {
            dst.put(bytes);
        }
    }

    pub fn decode(src: &mut Bytes) -> NonceReportData {
        let sequence_number = src.get_u8();
        let byte = src.get_u8();
        let mos = byte & 0x2 != 0;
        let sos = byte & 0x1 != 0;
        let mut entropy_input = None;
        if sos {
            entropy_input = Some(src.split_to(16));
        }
        NonceReportData { sequence_number, mos, sos, entropy_input }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Security2 {
    NonceGet(NonceGetData),
    NonceReport(NonceReportData),
    MessageEncapsulation(MessageEncapsulationData),
    KexGet,
    KexReport(KexData),
    KexSet(KexData),
    KexFail(KexFailData),
    PublicKeyReport(PublicKeyReportData),
}

impl Security2 {

    pub const CLASS : u8 = 0x9f;

    pub fn encode(&self, dst: &mut BytesMut) {
        match self {
            Security2::KexGet => {
                dst.put_u8(Command::KexGet.into());
            },
            Security2::KexReport(data) => {
                dst.put_u8(Command::KexReport.into());
                data.encode(dst);
            },
            Security2::KexSet(data) => {
                dst.put_u8(Command::KexSet.into());
                data.encode(dst);
            },
            Security2::KexFail(data) => {
                dst.put_u8(Command::KexFail.into());
                data.encode(dst);
            },
            Security2::PublicKeyReport(data) => {
                dst.put_u8(Command::PublicKeyReport.into());
                data.encode(dst);
            },
            _ => panic!("Not supported {:?}", self)
        }
    }

    pub fn decode(src: &mut Bytes) -> Security2 {
        let command: Option<Command> = FromPrimitive::from_u8(src.get_u8());
        return match command {
            Some(Command::KexGet) => {
                Security2::KexGet
            },
            Some(Command::KexReport) => {
                Security2::KexReport(KexData::decode(src))
            },
            Some(Command::KexSet) => {
                Security2::KexSet(KexData::decode(src))
            },
            Some(Command::KexFail) => {
                Security2::KexFail(KexFailData::decode(src))
            },
            Some(Command::PublicKeyReport) => {
                Security2::PublicKeyReport(PublicKeyReportData::decode(src))
            },
            _ => panic!("Do not support {:?}", command)
        }
    }
}