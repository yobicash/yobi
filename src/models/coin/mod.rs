use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::mac::YMACCode;
use libyobicash::amount::YAmount;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use serde_json;
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum YCoinKind {
    Coinbase=0,
    Transaction=1,
}

impl YCoinKind {
    pub fn to_bytes(&self) -> Vec<u8> {
        match *self {
            YCoinKind::Coinbase => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(0);
                buf.to_vec()
            }
            YCoinKind::Transaction => {
                let mut buf = BytesMut::new();
                buf.put_u32::<BigEndian>(1);
                buf.to_vec()
            }
        }
    }

    pub fn from_bytes(b: &[u8]) -> YHResult<YCoinKind> {
        if b.len() != 4 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        match BigEndian::read_u32(b) {
            0 => { Ok(YCoinKind::Coinbase) },
            1 => { Ok(YCoinKind::Transaction) },
            _ => { Err(YHErrorKind::InvalidCoinKind.into()) },
        }
    }
}

impl From<u32> for YCoinKind {
    fn from(n: u32) -> YCoinKind {
        match n {
            0 => YCoinKind::Coinbase,
            1 => YCoinKind::Transaction,
            _ => panic!("Invalid kind"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YCoin {
    pub date: YTime,
    pub sk: YSecretKey,
    pub kind: YCoinKind,
    pub id: YDigest64,
    pub idx: u32,
    pub has_data: bool,
    pub tag: Option<YMACCode>,
    pub amount: YAmount,
}

impl YCoin {
    pub fn new(
            date: YTime,
            sk: YSecretKey,
            kind: YCoinKind,
            id: YDigest64,
            idx: u32,
            has_data: bool,
            tag: Option<YMACCode>,
            amount: &YAmount) -> YHResult<YCoin> {
        if date > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        Ok(YCoin {
            date: date,
            sk: sk,
            kind: kind,
            id: id,
            idx: idx,
            has_data: has_data,
            tag: tag,
            amount: amount.clone(),
        })
    }

    pub fn check(&self) -> YHResult<()> {
        if self.date > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        if self.has_data && self.tag.is_none() {
            return Err(YHErrorKind::InvalidCoin.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(&self.date.to_bytes()[..]);
        buf.put(self.sk.to_bytes());
        buf.put(self.kind.to_bytes());
        buf.put(self.id.to_bytes());
        buf.put_u32::<BigEndian>(self.idx);
        buf.put_u32::<BigEndian>(self.has_data as u32);
        if let Some(tag) = self.tag {
            buf.put_u32::<BigEndian>(1);
            buf.put(tag.to_bytes());
        } else {
            buf.put_u32::<BigEndian>(0);
        }
        buf.put(self.amount.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YCoin> {
        if buf.len() < 145 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let date = YTime::from_bytes(b.get(0..8).unwrap())?;
        let sk = YSecretKey::from_bytes(b.get(8..72).unwrap())?;
        let kind = YCoinKind::from_bytes(b.get(72..76).unwrap())?;
        let id = YDigest64::from_bytes(b.get(76..140).unwrap())?;
        let idx = BigEndian::read_u32(b.get(140..144).unwrap());
        let has_data_n = BigEndian::read_u32(b.get(144..148).unwrap());
        let mut has_data = false;
        if has_data_n == 1 {
            has_data = true;
        } else if has_data_n != 0 {
            return Err(YHErrorKind::InvalidValue.into());
        };
        let has_tag = BigEndian::read_u32(b.get(148..152).unwrap());
        let mut tag = None;
        match has_tag {
            0 => {},
            1 => {
                tag = Some(YMACCode::from_bytes(b.get(152..184).unwrap())?);
            },
            _ => {
                return Err(YHErrorKind::InvalidValue.into());
            },
        }
        let amount = YAmount::from_bytes(b.get(184..).unwrap());
        let coin = YCoin {
            date: date,
            sk: sk,
            kind: kind,
            id: id,
            idx: idx,
            has_data: has_data,
            tag: tag,
            amount: amount,
        };
        coin.check()?;
        Ok(coin)
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YCoin> {
        let coin = serde_json::from_str(s)?;
        Ok(coin)
    }
}
