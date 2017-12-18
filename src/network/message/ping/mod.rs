use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YPingReq {
    pub method: YRPCMethod,
}

impl YPingReq {
    pub fn new() -> YPingReq {
        YPingReq {
            method: YRPCMethod::Ping,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::Ping {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        Ok(self.method.to_bytes())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPingReq> {
        if buf.len() != 4 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let ping_req = YPingReq {
            method: BigEndian::read_u32(buf).into(),
        };
        ping_req.check()?;
        Ok(ping_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YPingRes {
    pub method: YRPCMethod,
    pub public_key: YPublicKey,
    pub price: YAmount,
}

impl YPingRes {
    pub fn new(pk: YPublicKey, price: &YAmount) -> YHResult<YPingRes> {
        Ok(YPingRes {
            method: YRPCMethod::Ping,
            public_key: pk,
            price: price.clone(),
        })
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::Ping {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.public_key.to_bytes());
        buf.put(self.price.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPingRes> {
        if buf.len() < 69 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let public_key = YPublicKey::from_bytes(b.get(4..68).unwrap())?;
        let price = YAmount::from_bytes(b.get(68..).unwrap());
        let ping_res = YPingRes {
            method: method,
            public_key: public_key,
            price: price,
        };
        ping_res.check()?;
        Ok(ping_res)
    }
}
