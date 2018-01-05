use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YListTxAncestorsReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YListTxAncestorsReq {
    pub fn new(tx_id: YDigest64) -> YHResult<YListTxAncestorsReq> {
        let mut req = YListTxAncestorsReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListTxAncestors,
            tx_id: tx_id,
        };
        req.id = req.calc_id()?;
        Ok(req)
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
        if self.method != YRPCMethod::ListTxAncestors {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListTxAncestorsReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let ls_txs_req = YListTxAncestorsReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            tx_id: tx_id,
        };
        ls_txs_req.check()?;
        Ok(ls_txs_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YListTxAncestorsRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub count: u32,
    pub txs: Vec<YTransaction>,
}

impl YListTxAncestorsRes {
    pub fn new(txs: &Vec<YTransaction>) -> YHResult<YListTxAncestorsRes> {
        let mut res = YListTxAncestorsRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListTxAncestors,
            count: txs.len() as u32,
            txs: txs.clone(),
        };
        res.id = res.calc_id()?;
        Ok(res)
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
        if self.method != YRPCMethod::ListTxAncestors {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        if self.txs.len() != self.count as usize {
            return Err(YHErrorKind::InvalidLength.into());
        }
        for tx in self.txs.clone() {
            tx.check()?
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let tx_buf = self.txs[i].to_bytes()?;
            let tx_size = tx_buf.len();
            buf.put_u32::<BigEndian>(tx_size as u32);
            buf.put(tx_buf);
        }
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let tx_buf = self.txs[i].to_bytes()?;
            let tx_size = tx_buf.len();
            buf.put_u32::<BigEndian>(tx_size as u32);
            buf.put(tx_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListTxAncestorsRes> {
        if buf.len() < 96 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let count = BigEndian::read_u32(b.get(92..96).unwrap());
        let mut tx_buf = BytesMut::new();
        tx_buf.extend_from_slice(b.get(96..).unwrap());
        let mut txs = Vec::new();
        for i in 0..count as usize {
            let size = BigEndian::read_u32(tx_buf.get(i..i+4).unwrap()) as usize;
            txs.push(YTransaction::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        let ls_txs_res = YListTxAncestorsRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            count: count,
            txs: txs,
        };
        ls_txs_res.check()?;
        Ok(ls_txs_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YListTxDescendantsReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YGetTxReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YGetTxReq {
    pub fn new(tx_id: YDigest64) -> YHResult<YGetTxReq> {
        let mut req = YGetTxReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetTx,
            tx_id: tx_id,
        };
        req.id = req.calc_id()?;
        Ok(req)
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
        if self.method != YRPCMethod::GetTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetTxReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let get_tx_req = YGetTxReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            tx_id: tx_id,
        };
        get_tx_req.check()?;
        Ok(get_tx_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YGetTxRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx: YTransaction,
}

impl YGetTxRes {
    pub fn new(tx: &YTransaction) -> YHResult<YGetTxRes> {
        let mut res = YGetTxRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetTx,
            tx: tx.clone(),
        };
        res.id = res.calc_id()?;
        Ok(res)
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
        if self.method != YRPCMethod::GetTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        self.tx.check()?;
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx.to_bytes()?);
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put(self.tx.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetTxRes> {
        if buf.len() < 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let tx = YTransaction::from_bytes(b.get(92..).unwrap())?;
        let get_tx_res = YGetTxRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            tx: tx,
        };
        get_tx_res.check()?;
        Ok(get_tx_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YConfirmTxReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YConfirmTxReq {
    pub fn new(tx_id: YDigest64) -> YHResult<YConfirmTxReq> {
        let mut req = YConfirmTxReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ConfirmTx,
            tx_id: tx_id,
        };
        req.id = req.calc_id()?;
        Ok(req)
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
        if self.method != YRPCMethod::ConfirmTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YConfirmTxReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let conf_tx_req = YConfirmTxReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            tx_id: tx_id,
        };
        conf_tx_req.check()?;
        Ok(conf_tx_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YConfirmTxRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub ack: bool,
    pub cb: YCoinbase,
}

impl YConfirmTxRes {
    pub fn new(ack: bool, cb: &YCoinbase) -> YHResult<YConfirmTxRes> {
        let mut res = YConfirmTxRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ConfirmTx,
            ack: ack,
            cb: cb.clone(),
        };
        res.id = res.calc_id()?;
        Ok(res)
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
        if self.method != YRPCMethod::ConfirmTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        self.cb.check()?;
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.ack as u32);
        buf.put(self.cb.to_bytes()?);
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.ack as u32);
        buf.put(self.cb.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YConfirmTxRes> {
        if buf.len() < 96 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let ack_n = BigEndian::read_u32(b.get(92..96).unwrap());
        let mut ack = false;
        match ack_n {
            0 => {},
            1 => { ack = true; },
            _ => { return Err(YHErrorKind::Other("Invalid ack".to_string()).into()); }
        }
        let cb = YCoinbase::from_bytes(b.get(96..).unwrap())?;
        let confirm_tx_res = YConfirmTxRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            ack: ack,
            cb: cb,
        };
        confirm_tx_res.check()?;
        Ok(confirm_tx_res)
    }
}
