use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YErrorRes {
    pub method: YMethod,
    pub message: String,
}

impl YErrorRes {
    pub fn new(method: YMethod, message: String) -> YErrorRes {
        YErrorRes {
            method: method,
            message: message,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.message.len() > 20 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.message.as_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YErrorRes> {
        if buf.len() > 44 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let message = String::from_utf8_lossy(b.get(8..).unwrap()).into_owned();
        Ok(YErrorRes {
            method: method,
            message: message,
        })
    }
}
