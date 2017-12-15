use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::time::YTime;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPeer {
    pub ip: [u8; 4],
    pub port: u16,
    pub last_time: YTime,
}

impl Default for YPeer {
    fn default() -> YPeer {
        let ip = [0, 0, 0, 0];
        let port = 2112;
        YPeer::new(ip, port)
    }
}

impl YPeer {
    pub fn new(ip: [u8; 4], port: u16) -> YPeer {
        YPeer {
            ip: ip,
            port: port,
            last_time: YTime::now(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.last_time > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(&self.ip[..]);
        buf.put_u16::<BigEndian>(self.port);
        buf.put(&self.last_time.to_bytes()[..]);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPeer> {
        if buf.len() != 14 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ip = [0u8; 4];
        for i in 0..4 {
            ip[i] = b[i]
        }
        let port = BigEndian::read_u16(b.get(4..6).unwrap());
        let last_time = YTime::from_bytes(b.get(6..).unwrap())?;
        let peer = YPeer {
            ip: ip,
            port: port,
            last_time: last_time,
        };
        peer.check()?;
        Ok(peer)
    }
}
