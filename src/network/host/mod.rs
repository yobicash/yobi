use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use serde_json;
use errors::*;
use std::net::Ipv4Addr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YHost {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl Default for YHost {
    fn default() -> YHost {
        YHost {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 2112,
        }
    }
}

impl YHost {
    pub fn new(addr: [u8; 4], port: u16) -> YHost {
        YHost {
            address: Ipv4Addr::from(addr),
            port: port,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        buf.put(&self.address.octets()[..]);
        buf.put_u16::<BigEndian>(self.port);
        buf.to_vec()
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YHost> {
        if buf.len() != 6 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let addr = Ipv4Addr::new(b[0], b[1], b[2], b[3]);
        let port = BigEndian::read_u16(b.get(4..).unwrap());
        let host = YHost {
            address: addr,
            port: port,
        };
        Ok(host)
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YHost> {
        let host = serde_json::from_str(s)?;
        Ok(host)
    }
}
