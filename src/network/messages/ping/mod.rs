use libyobicash::utils::time::YTime;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use libyobicash::errors::*;
use network::method::YMethod;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPingResponse {
    pub method: YMethod,
    pub time: YTime,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPingResponse {
    pub fn new(pk: YPublicKey, price: &YAmount) -> YResult<YPingResponse> {
        Ok(YPing {
            method: YMethod::Ping,
            time: YTime::now(),
            public_key: pk.to_bytes()?,
            price: price.to_string()?,
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        unreachable!()
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPingResponse> {
        unreachable!()
    }
}
