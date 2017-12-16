use libyobicash::errors::YErrorKind as LibErrorKind;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;
use store::common::YStoreBuck;
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum YBucket {
    Transactions=0,
    Data=1,
    Coinbases=2,
    Ucoins=3,
    Scoins=4,
    Wallets=5,
    PeersByIp=6,
    PeersByLastTime=7,
    Unknown,
}

impl Default for YBucket {
    fn default() -> YBucket {
        YBucket::Unknown
    }
}

impl From<u32> for YBucket {
    fn from(n: u32) -> YBucket {
        match n {
            0 => YBucket::Transactions,
            1 => YBucket::Data,
            2 => YBucket::Coinbases,
            3 => YBucket::Ucoins,
            4 => YBucket::Scoins,
            5 => YBucket::Wallets,
            6 => YBucket::PeersByIp,
            7 => YBucket::PeersByLastTime,
            _ => YBucket::Unknown,
        }
    }
}

impl YBucket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put_u32::<BigEndian>(*self as u32);
        buf.to_vec()
    }

    pub fn to_store_buck(&self) -> YStoreBuck {
        self.to_bytes()
    }

    pub fn from_bytes(b: &[u8]) -> YHResult<YBucket> {
        if b.len() != 4 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        Ok(BigEndian::read_u32(b).into())
    }
}
