use libyobicash::errors::YErrorKind as LibErrorKind;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;
use store::common::YStoreBuck;
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum YBucket {
    Transactions=0,
    Coinbases=1,
    Data=2,
    UTXO=3,
    Wallets=4,
    PeersByIp=5,
    PeersByLastTime=6,
    Keys=7,
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
            1 => YBucket::Coinbases,
            2 => YBucket::Data,
            3 => YBucket::UTXO,
            4 => YBucket::Wallets,
            5 => YBucket::PeersByIp,
            6 => YBucket::PeersByLastTime,
            7 => YBucket::Keys,
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
