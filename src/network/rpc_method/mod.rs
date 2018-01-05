use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum YRPCMethod {
    Ping=0,
    ListPeers=1,
    ListData=2,
    GetData=3,
    ListTxAncestors=4,
    GetTx=5,
    ConfirmTx=6,
    GetCb=7,
    Unknown,
}

impl Default for YRPCMethod {
    fn default() -> YRPCMethod {
        YRPCMethod::Ping
    }
}

impl From<u32> for YRPCMethod {
    fn from(n: u32) -> YRPCMethod {
        match n {
            0 => YRPCMethod::Ping,
            1 => YRPCMethod::ListPeers,
            2 => YRPCMethod::ListData,
            3 => YRPCMethod::GetData,
            4 => YRPCMethod::ListTxAncestors,
            5 => YRPCMethod::GetTx,
            6 => YRPCMethod::ConfirmTx,
            7 => YRPCMethod::GetCb,
            _ => YRPCMethod::Unknown,
        }
    }
}

impl YRPCMethod {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u32::<BigEndian>(*self as u32);
        buf.to_vec()
    }

    pub fn from_bytes(b: &[u8]) -> YHResult<YRPCMethod> {
        if b.len() != 4 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        Ok(BigEndian::read_u32(b).into())
    }
}
