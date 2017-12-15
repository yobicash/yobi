use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::errors::YError as LibError;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use std::convert::From;
use network::method::*;
use version::*;
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum YMessageKind {
    Request,
    Response,
}

impl YMessageKind {
    pub fn to_bytes(&self) -> Vec<u8> {
        match *self {
            YMessageKind::Request => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(0);
                buf.to_vec()
            }
            YMessageKind::Response => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(1);
                buf.to_vec()
            }
        }
    }

    pub fn from_bytes(b: &[u8]) -> YHResult<YMessageKind> {
        if b.len() != 4 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        match BigEndian::read_u32(b) {
            0 => { Ok(YMessageKind::Request) },
            1 => { Ok(YMessageKind::Response) },
            _ => { Err(YHErrorKind::InvalidMessageKind.into()) },
        }
    }
}

impl From<u32> for YMessageKind {
    fn from(n: u32) -> YMessageKind {
        match n {
            0 => YMessageKind::Request,
            1 => YMessageKind::Response,
            _ => panic!("Invalid kind"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum YMessageStatus {
    Failure,
    Success,
}

impl YMessageStatus {
    pub fn to_bytes(&self) -> Vec<u8> {
        match *self {
            YMessageStatus::Failure => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(0);
                buf.to_vec()
            }
            YMessageStatus::Success => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(1);
                buf.to_vec()
            }
        }
    }

    pub fn from_bytes(b: &[u8]) -> YHResult<YMessageStatus> {
        if b.len() != 4 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        match BigEndian::read_u32(b) {
            0 => { Ok(YMessageStatus::Failure) },
            1 => { Ok(YMessageStatus::Success) },
            _ => { Err(YHErrorKind::InvalidMessageStatus.into()) },
        }
    }
}

impl From<u32> for YMessageStatus {
    fn from(n: u32) -> YMessageStatus {
        match n {
            0 => YMessageStatus::Failure,
            1 => YMessageStatus::Success,
            _ => panic!("Invalid status"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YMessage {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YMethod,
    pub kind: YMessageKind,
    pub status: YMessageStatus,
    pub payload: Vec<u8>,
}

impl YMessage {
    pub fn new(method: YMethod, kind: YMessageKind, status: YMessageStatus, payload: &Vec<u8>) -> YHResult<YMessage> {
        let mut msg = YMessage {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: method,
            kind: kind,
            status: status,
            payload: payload.clone(),
        };
        msg.id = msg.calc_id()?;
        Ok(msg)
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
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.kind.to_bytes());
        buf.put(self.status.to_bytes());
        buf.put(self.payload.clone());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.kind.to_bytes());
        buf.put(self.status.to_bytes());
        buf.put(self.payload.clone());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YMessage> { 
        if buf.len() < 100 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let method = BigEndian::read_u32(b.get(84..88).unwrap()).into();
        let nonce = BigEndian::read_u32(b.get(88..92).unwrap());
        let kind = BigEndian::read_u32(b.get(92..96).unwrap()).into();
        let status = BigEndian::read_u32(b.get(96..100).unwrap()).into();
        let mut payload = Vec::new();
        payload.extend_from_slice(b.get(100..).unwrap());
        let msg = YMessage {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            kind: kind,
            status: status,
            payload: payload,
        };
        msg.check()?;
        Ok(msg)
    }
}
