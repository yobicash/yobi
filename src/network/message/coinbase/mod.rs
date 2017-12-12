use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::coinbase::YCoinbase;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YGetCbReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub cb_id: YDigest64,
}

impl YGetCbReq {
    pub fn new(cb_id: YDigest64) -> YGetCbReq {
        YGetCbReq {
            version: default_version(),
            method: YMethod::GetCb,
            id: YRandom::u32(),
            time: YTime::now(),
            cb_id: cb_id,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetCb {
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
        buf.put(self.cb_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetCbReq> {
        if buf.len() != 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_cbs_req = YGetCbReq::default();
        ls_cbs_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_cbs_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_cbs_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_cbs_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_cbs_req.cb_id = YDigest64::from_bytes(b.get(44..).unwrap())?;
        ls_cbs_req.check()?;
        Ok(ls_cbs_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YGetCbRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub cb: YCoinbase,
}

impl YGetCbRes {
    pub fn new(id: u32, cb: &YCoinbase) -> YGetCbRes {
        YGetCbRes {
            version: default_version(),
            method: YMethod::GetCb,
            id: id,
            time: YTime::now(),
            cb: cb.clone(),
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetCb {
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
        buf.put(self.cb.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetCbRes> {
        if buf.len() < 44 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut get_cb_res = YGetCbRes::default();
        get_cb_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        get_cb_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        get_cb_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        get_cb_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        get_cb_res.cb = YCoinbase::from_bytes(b.get(44..).unwrap())?;
        get_cb_res.check()?;
        Ok(get_cb_res)
    }
}
