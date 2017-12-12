use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum YMethod {
    Ping=0,
    ListPeers=1,
    ListData=2,
    GetData=3,
    ListTxAncestors=4,
    GetTx=5,
    ConfirmTx=6,
    ListCbs=7,
    GetCb=8,
    Unknown,
}

impl Default for YMethod {
    fn default() -> YMethod {
        YMethod::Unknown
    }
}

impl From<u32> for YMethod {
    fn from(n: u32) -> YMethod {
        match n {
            0 => YMethod::Ping,
            1 => YMethod::ListPeers,
            2 => YMethod::ListData,
            3 => YMethod::GetData,
            4 => YMethod::ListTxAncestors,
            5 => YMethod::GetTx,
            6 => YMethod::ConfirmTx,
            7 => YMethod::ListCbs,
            8 => YMethod::GetCb,
            _ => YMethod::Unknown,
        }
    }
}

impl YMethod {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u32::<BigEndian>(*self as u32);
        buf.to_vec()
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YMethod> {
        if b.len() != 4 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(BigEndian::read_u32(b).into())
    }
}
