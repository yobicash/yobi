use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YPingReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
}

impl YPingReq {
    pub fn new() -> YHResult<YPingReq> {
        let mut req = YPingReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::Ping,
        };
        req.id = req.calc_id()?;
        Ok(req)
    }

    pub fn check(&self) -> YHResult<()> {
        if self.id != self.calc_id()? {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidChecksum).into());
        }
        if self.version.major() > default_version().major() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidVersion(self.version.to_string())).into());
        }
        if self.time > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        if self.method != YRPCMethod::Ping {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPingReq> {
        if buf.len() != 92 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let ping_req = YPingReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
        };
        ping_req.check()?;
        Ok(ping_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YPingRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPingRes {
    pub fn new(pk: YPublicKey, price: &YAmount) -> YHResult<YPingRes> {
        let mut res = YPingRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::Ping,
            public_key: pk,
            price: price.clone(),
        };
        res.id = res.calc_id()?;
        Ok(res)
    }

    pub fn check(&self) -> YHResult<()> {
        if self.id != self.calc_id()? {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidChecksum).into());
        }
        if self.version.major() > default_version().major() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidVersion(self.version.to_string())).into());
        }
        if self.time > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        if self.method != YRPCMethod::Ping {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.public_key.to_bytes());
        buf.put(self.price.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.public_key.to_bytes());
        buf.put(self.price.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPingRes> {
        if buf.len() < 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let public_key = YPublicKey::from_bytes(b.get(92..156).unwrap())?;
        let price = YAmount::from_bytes(b.get(156..).unwrap());
        let ping_res = YPingRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            public_key: public_key,
            price: price,
        };
        ping_res.check()?;
        Ok(ping_res)
    }
}
