use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YErrorRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub message: String,
}

impl YErrorRes {
    pub fn new(method: YRPCMethod, message: String) -> YHResult<YErrorRes> {
        let mut res = YErrorRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: method,
            message: message,
        };
        res.id = res.calc_id()?;
        Ok(res)
    }

    pub fn from_error(method: YRPCMethod, err: YHError) -> YHResult<YErrorRes> {
        let msg = String::from(err.description());
        YErrorRes::new(method, msg)
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
        if self.message.len() > 20 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.message.as_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.message.as_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YErrorRes> {
        if buf.len() < 92 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let message = String::from_utf8_lossy(b.get(92..).unwrap()).into_owned();
        Ok(YErrorRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            message: message,
        })
    }
}
