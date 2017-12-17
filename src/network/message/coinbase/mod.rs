use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::coinbase::YCoinbase;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YGetCbReq {
    pub method: YRPCMethod,
    pub cb_id: YDigest64,
}

impl YGetCbReq {
    pub fn new(cb_id: YDigest64) -> YGetCbReq {
        YGetCbReq {
            method: YRPCMethod::GetCb,
            cb_id: cb_id,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::GetCb {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.cb_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetCbReq> {
        if buf.len() != 68 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let cb_id = YDigest64::from_bytes(b.get(4..).unwrap())?;
        let get_cb_req = YGetCbReq {
            method: method,
            cb_id: cb_id,
        };
        get_cb_req.check()?;
        Ok(get_cb_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YGetCbRes {
    pub method: YRPCMethod,
    pub cb: YCoinbase,
}

impl YGetCbRes {
    pub fn new(cb: &YCoinbase) -> YGetCbRes {
        YGetCbRes {
            method: YRPCMethod::GetCb,
            cb: cb.clone(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::GetCb {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.cb.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetCbRes> {
        if buf.len() < 108 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        let cb = YCoinbase::from_bytes(b.get(44..).unwrap())?;
        let get_cb_res = YGetCbRes {
            method: method,
            cb: cb,
        };
        get_cb_res.check()?;
        Ok(get_cb_res)
    }
}
