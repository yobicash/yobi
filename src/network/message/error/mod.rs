use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YErrorRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub message: String,
}

impl YErrorRes {
    pub fn new(method: YMethod, id: u32, message: String) -> YErrorRes {
        YErrorRes {
            version: default_version(),
            method: method,
            id: id,
            time: YTime::now(),
            message: message,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
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
        let msg_buf = self.message.as_bytes();
        buf.put_u32::<BigEndian>(msg_buf.len() as u32);
        buf.put(msg_buf);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YErrorRes> {
        if buf.len() != 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut err_res = YErrorRes::default();
        err_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        err_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        err_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        err_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        let msg_size = BigEndian::read_u32(b.get(44..48).unwrap()) as usize;
        err_res.message = String::from_utf8_lossy(b.get(48..48+msg_size).unwrap()).into_owned();
        err_res.check()?;
        Ok(err_res)
    }
}
