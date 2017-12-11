use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YPingReq {
    pub id: u32,
    pub method: YMethod,
    pub time: YTime,
}

impl YPingReq {
    pub fn new() -> YPingReq {
        YPingReq {
            id: YRandom::u32(),
            method: YMethod::Ping,
            time: YTime::now(),
        }
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(self.method.to_bytes());
        buf.put(&self.time.to_bytes()[..]);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPingReq> {
        if buf.len() != 16 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ping_req = YPingReq::default();
        ping_req.id = BigEndian::read_u32(b.get(0..4).unwrap());
        ping_req.method = BigEndian::read_u32(b.get(4..8).unwrap()).into();
        ping_req.time = YTime::from_bytes(b.get(8..).unwrap())?;
        Ok(ping_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YPingRes {
    pub id: u32,
    pub method: YMethod,
    pub time: YTime,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPingRes {
    pub fn new(id: u32, pk: YPublicKey, price: &YAmount) -> YResult<YPingRes> {
        Ok(YPingRes {
            id: id,
            method: YMethod::Ping,
            time: YTime::now(),
            public_key: pk,
            price: price.clone(),
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(self.method.to_bytes());
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.public_key.to_bytes());
        buf.put(self.price.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPingRes> {
        if buf.len() != 16 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ping_res = YPingRes::default();
        ping_res.id = BigEndian::read_u32(b.get(0..4).unwrap());
        ping_res.method = BigEndian::read_u32(b.get(4..8).unwrap()).into();
        ping_res.time = YTime::from_bytes(b.get(8..16).unwrap())?;
        ping_res.public_key = YPublicKey::from_bytes(b.get(16..48).unwrap())?;
        ping_res.price = YAmount::from_bytes(b.get(48..).unwrap())?;
        Ok(ping_res)
    }
}
