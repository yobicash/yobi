use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct YAddress {
    pub host: [u8; 4],
    pub port: u16,
}

impl Default for YAddress {
    fn default() -> YAddress {
        YAddress {
            host: [127, 0, 0, 1],
            port: 2112,
        }
    }
}

impl YAddress {
    pub fn new(host: [u8; 4], port: u16) -> YAddress {
        YAddress {
            host: host,
            port: port,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put(&self.host[..]);
        buf.put_u16::<BigEndian>(self.port);
        buf.to_vec()
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YAddress> {
        if buf.len() != 6 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut host = [0u8; 4];
        for i in 0..4 {
            host[i] = b[i]
        }
        let port = BigEndian::read_u16(b.get(4..).unwrap());
        let host = YAddress {
            host: host,
            port: port,
        };
        Ok(host)
    }
}
