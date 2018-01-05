use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use serde_json;
use errors::*;
use std::net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YHost(pub SocketAddr);

impl Default for YHost {
    fn default() -> YHost {
        YHost::new([127, 0, 0 , 1], 2112)
    }
}

impl YHost {
    pub fn new(addr: [u8; 4], port: u16) -> YHost {
        let ip = Ipv4Addr::from(addr);
        YHost(SocketAddr::V4(SocketAddrV4::new(ip, port)))
    }

    pub fn internal(&self) -> SocketAddr {
        self.0.clone()
    }

    pub fn ip(&self) -> YHResult<Ipv4Addr> {
        match self.0.ip() {
            IpAddr::V4(ip) => { Ok(ip) },
            _ => {
                return Err(YHErrorKind::InvalidIp.into());
            }
        }
    }

    pub fn port(&self) -> u16 {
        self.0.port()
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(&self.ip()?.octets()[..]);
        buf.put_u16::<BigEndian>(self.port());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YHost> {
        if buf.len() != 6 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let addr: [u8; 4] = [b[0], b[1], b[2], b[3]];
        let port = BigEndian::read_u16(b.get(4..).unwrap());
        let host = YHost::new(addr, port);
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
