use libyobicash::utils::time::YTime;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use libyobicash::errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPing {
    pub time: YTime,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPing {
    pub fn new(pk: YPublicKey, price: &YAmount) -> YResult<YPing> {
        Ok(YPing {
            time: YTime::now(),
            public_key: pk.to_bytes()?,
            price: price.to_string()?,
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        unreachable!()
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPing> {
        unreachable!()
    }

    pub fn to_hex(&self) -> YResult<String> {
        unreachable!()
    }

    pub fn from_hex(buf: &str) -> YResult<YPing> {
        unreachable!()
    }
}
