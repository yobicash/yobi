use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YListTxAncestorsReq {
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YListTxAncestorsReq {
    pub fn new(tx_id: YDigest64) -> YListTxAncestorsReq {
        YListTxAncestorsReq {
            method: YRPCMethod::ListTxAncestors,
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::ListTxAncestors {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListTxAncestorsReq> {
        if buf.len() != 68 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(4..).unwrap())?;
        let ls_txs_req = YListTxAncestorsReq {
            method: method,
            tx_id: tx_id,
        };
        ls_txs_req.check()?;
        Ok(ls_txs_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YListTxAncestorsRes {
    pub method: YRPCMethod,
    pub count: u32,
    pub txs: Vec<YTransaction>,
}

impl YListTxAncestorsRes {
    pub fn new(txs: &Vec<YTransaction>) -> YListTxAncestorsRes {
        YListTxAncestorsRes {
            method: YRPCMethod::ListTxAncestors,
            count: txs.len() as u32,
            txs: txs.clone(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::ListTxAncestors {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        if self.txs.len() != self.count as usize {
            return Err(YHErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
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
        if buf.len() < 48 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let count = BigEndian::read_u32(b.get(4..8).unwrap());
        let mut tx_buf = BytesMut::new();
        tx_buf.extend_from_slice(b.get(8..).unwrap());
        let mut txs = Vec::new();
        for i in 0..count as usize {
            let size = BigEndian::read_u32(tx_buf.get(i..i+4).unwrap()) as usize;
            txs.push(YTransaction::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        let ls_txs_res = YListTxAncestorsRes {
            method: method,
            count: count,
            txs: txs,
        };
        ls_txs_res.check()?;
        Ok(ls_txs_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YGetTxReq {
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YGetTxReq {
    pub fn new(tx_id: YDigest64) -> YGetTxReq {
        YGetTxReq {
            method: YRPCMethod::GetTx,
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::GetTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetTxReq> {
        if buf.len() != 68 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(44..).unwrap())?;
        let get_tx_req = YGetTxReq {
            method: method,
            tx_id: tx_id,
        };
        get_tx_req.check()?;
        Ok(get_tx_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YGetTxRes {
    pub method: YRPCMethod,
    pub tx: YTransaction,
}

impl YGetTxRes {
    pub fn new(tx: &YTransaction) -> YGetTxRes {
        YGetTxRes {
            method: YRPCMethod::GetTx,
            tx: tx.clone(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::GetTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.tx.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetTxRes> {
        if buf.len() < 108 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let tx = YTransaction::from_bytes(b.get(4..).unwrap())?;
        let get_tx_res = YGetTxRes {
            method: method,
            tx: tx,
        };
        get_tx_res.check()?;
        Ok(get_tx_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YConfirmTxReq {
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YConfirmTxReq {
    pub fn new(tx_id: YDigest64) -> YConfirmTxReq {
        YConfirmTxReq {
            method: YRPCMethod::ConfirmTx,
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::ConfirmTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YConfirmTxReq> {
        if buf.len() != 68 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(4..).unwrap())?;
        let conf_tx_req = YConfirmTxReq {
            method: method,
            tx_id: tx_id,
        };
        conf_tx_req.check()?;
        Ok(conf_tx_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YConfirmTxRes {
    pub method: YRPCMethod,
    pub ack: bool,
    pub cb: YCoinbase,
}

impl YConfirmTxRes {
    pub fn new(ack: bool, cb: &YCoinbase) -> YConfirmTxRes {
        YConfirmTxRes {
            method: YRPCMethod::ConfirmTx,
            ack: ack,
            cb: cb.clone(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.method != YRPCMethod::ConfirmTx {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.ack as u32);
        buf.put(self.cb.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YConfirmTxRes> {
        if buf.len() < 112 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let ack_n = BigEndian::read_u32(b.get(4..8).unwrap());
        let mut ack = false;
        match ack_n {
            0 => {},
            1 => { ack = true; },
            _ => { return Err(YHErrorKind::Other("Invalid ack".to_string()).into()); }
        }
        let cb = YCoinbase::from_bytes(b.get(8..).unwrap())?;
        let confirm_tx_res = YConfirmTxRes {
            method: method,
            ack: ack,
            cb: cb,
        };
        confirm_tx_res.check()?;
        Ok(confirm_tx_res)
    }
}
