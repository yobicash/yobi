use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum YMethod {
    Ping=0,
    ListPeers=1,
    ListData=2,
    GetData=3,
    ListTransactions=4,
    GetTransaction=5,
    ConfirmTransaction=6,
    ListCoinbases=7,
    GetCoinbase=8,
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
            4 => YMethod::ListTransactions,
            5 => YMethod::GetTransaction,
            6 => YMethod::ConfirmTransaction,
            7 => YMethod::ListCoinbases,
            8 => YMethod::GetCoinbase,
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
