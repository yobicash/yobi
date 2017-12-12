use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YPingReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
}

impl YPingReq {
    pub fn new() -> YPingReq {
        YPingReq {
            version: default_version(),
            method: YMethod::Ping,
            id: YRandom::u32(),
            time: YTime::now(),
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::Ping {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(&self.time.to_bytes()[..]);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPingReq> {
        if buf.len() != 44 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ping_req = YPingReq::default();
        ping_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ping_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ping_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ping_req.time = YTime::from_bytes(b.get(36..).unwrap())?;
        ping_req.check()?;
        Ok(ping_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YPingRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPingRes {
    pub fn new(id: u32, pk: YPublicKey, price: &YAmount) -> YResult<YPingRes> {
        Ok(YPingRes {
            version: default_version(),
            method: YMethod::Ping,
            id: id,
            time: YTime::now(),
            public_key: pk,
            price: price.clone(),
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::Ping {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.public_key.to_bytes());
        buf.put(self.price.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPingRes> {
        if buf.len() < 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ping_res = YPingRes::default();
        ping_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ping_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ping_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ping_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ping_res.public_key = YPublicKey::from_bytes(b.get(44..108).unwrap())?;
        ping_res.price = YAmount::from_bytes(b.get(108..).unwrap())?;
        ping_res.check()?;
        Ok(ping_res)
    }
}
