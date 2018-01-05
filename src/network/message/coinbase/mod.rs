use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::coinbase::YCoinbase;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YGetCbReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub cb_id: YDigest64,
}

impl YGetCbReq {
    pub fn new(cb_id: YDigest64) -> YHResult<YGetCbReq> {
        let mut req = YGetCbReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetCb,
            cb_id: cb_id,
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
        if self.method != YRPCMethod::GetCb {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.cb_id.to_bytes());
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
        buf.put(self.cb_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetCbReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let cb_id = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let get_cb_req = YGetCbReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            cb_id: cb_id,
        };
        get_cb_req.check()?;
        Ok(get_cb_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YGetCbRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub cb: YCoinbase,
}

impl YGetCbRes {
    pub fn new(cb: &YCoinbase) -> YHResult<YGetCbRes> {
        let mut res = YGetCbRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetCb,
            cb: cb.clone(),
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
        if self.method != YRPCMethod::GetCb {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        self.cb.check()?;
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.cb.to_bytes()?);
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
        buf.put(self.cb.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetCbRes> {
        if buf.len() < 192 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let cb = YCoinbase::from_bytes(b.get(92..).unwrap())?;
        let get_cb_res = YGetCbRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            cb: cb,
        };
        get_cb_res.check()?;
        Ok(get_cb_res)
    }
}
