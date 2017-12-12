use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::transaction::YTransaction;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YListTxAncestorsReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub tx_id: YDigest64,
}

impl YListTxAncestorsReq {
    pub fn new(tx_id: YDigest64) -> YListTxAncestorsReq {
        YListTxAncestorsReq {
            version: default_version(),
            method: YMethod::ListTxAncestors,
            id: YRandom::u32(),
            time: YTime::now(),
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::ListTxAncestors {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
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
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListTxAncestorsReq> {
        if buf.len() != 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_txs_req = YListTxAncestorsReq::default();
        ls_txs_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_txs_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_txs_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_txs_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_txs_req.tx_id = YDigest64::from_bytes(b.get(44..).unwrap())?;
        ls_txs_req.check()?;
        Ok(ls_txs_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YListTxAncestorsRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub count: u32,
    pub tx: Vec<YTransaction>,
}

impl YListTxAncestorsRes {
    pub fn new(id: u32, count: u32, tx: &Vec<YTransaction>) -> YResult<YListTxAncestorsRes> {
        if tx.len() != count as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YListTxAncestorsRes {
            version: default_version(),
            method: YMethod::ListTxAncestors,
            id: id,
            time: YTime::now(),
            count: count,
            tx: tx.clone(),
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::ListTxAncestors {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        if self.tx.len() != self.count as usize {
            return Err(YErrorKind::InvalidLength.into());
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
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let tx_buf = self.tx[i].to_bytes()?;
            let tx_size = tx_buf.len();
            buf.put_u32::<BigEndian>(tx_size as u32);
            buf.put(tx_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListTxAncestorsRes> {
        if buf.len() < 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_txs_res = YListTxAncestorsRes::default();
        ls_txs_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_txs_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_txs_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_txs_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_txs_res.count = BigEndian::read_u32(b.get(44..48).unwrap());
        let mut tx_buf = BytesMut::new();
        tx_buf.extend_from_slice(b.get(48..).unwrap());
        for i in 0..ls_txs_res.count as usize {
            let size = BigEndian::read_u32(tx_buf.get(i..i+4).unwrap()) as usize;
            ls_txs_res.tx.push(YTransaction::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        Ok(ls_txs_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YGetTxReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub tx_id: YDigest64,
}

impl YGetTxReq {
    pub fn new(tx_id: YDigest64) -> YGetTxReq {
        YGetTxReq {
            version: default_version(),
            method: YMethod::GetTx,
            id: YRandom::u32(),
            time: YTime::now(),
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetTx {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
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
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetTxReq> {
        if buf.len() != 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_txs_req = YGetTxReq::default();
        ls_txs_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_txs_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_txs_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_txs_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_txs_req.tx_id = YDigest64::from_bytes(b.get(44..).unwrap())?;
        ls_txs_req.check()?;
        Ok(ls_txs_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YGetTxRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub tx: YTransaction,
}

impl YGetTxRes {
    pub fn new(id: u32, count: u32, tx: &YTransaction) -> YGetTxRes {
        YGetTxRes {
            version: default_version(),
            method: YMethod::GetTx,
            id: id,
            time: YTime::now(),
            tx: tx.clone(),
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetTx {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
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
        buf.put(self.tx.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetTxRes> {
        if buf.len() < 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut get_tx_res = YGetTxRes::default();
        get_tx_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        get_tx_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        get_tx_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        get_tx_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        get_tx_res.tx = YTransaction::from_bytes(b.get(44..).unwrap())?;
        get_tx_res.check()?;
        Ok(get_tx_res)
    }
}
