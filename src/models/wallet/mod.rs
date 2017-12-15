use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::amount::YAmount;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YCoin {
    pub date: YTime,
    pub sk: YSecretKey,
    pub tx_id: YDigest64,
    pub idx: u32,
    pub amount: YAmount,
}

impl YCoin {
    pub fn new(date: YTime, sk: YSecretKey, tx_id: YDigest64, idx: u32, amount: &YAmount) -> YHResult<YCoin> {
        if date > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        Ok(YCoin {
            date: date,
            sk: sk,
            tx_id: tx_id,
            idx: idx,
            amount: amount.clone(),
        })
    }

    pub fn check(&self) -> YHResult<()> {
        if self.date > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        buf.put(&self.date.to_bytes()[..]);
        buf.put(self.sk.to_bytes());
        buf.put(self.tx_id.to_bytes());
        buf.put_u32::<BigEndian>(self.idx);
        buf.put(self.amount.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YCoin> {
        if buf.len() < 141 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let date = YTime::from_bytes(b.get(0..8).unwrap())?;
        let sk = YSecretKey::from_bytes(b.get(8..72).unwrap())?;
        let tx_id = YDigest64::from_bytes(b.get(72..136).unwrap())?;
        let idx = BigEndian::read_u32(b.get(136..140).unwrap());
        let amount = YAmount::from_bytes(b.get(140..).unwrap())?;
        let coin = YCoin {
            date: date,
            sk: sk,
            tx_id: tx_id,
            idx: idx,
            amount: amount,
        };
        coin.check()?;
        Ok(coin)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YWallet {
    pub name: String,
    pub balance: YAmount,
    pub scoins: Vec<YCoin>,
    pub ucoins: Vec<YCoin>,
}

impl YWallet {
    pub fn new(name: &str) -> YWallet {
        YWallet {
            name: String::from(name),
            balance: YAmount::zero(),
            scoins: Vec::new(),
            ucoins: Vec::new(),
        }
    }

    pub fn check(&self) -> YHResult<()> {
        let scoins_len = self.scoins.len();
        for i in 0..scoins_len {
            self.scoins[i].check()?;
        }
        let mut ucoins_balance = YAmount::zero();
        for i in 0..self.ucoins.len() {
            let ucoin = self.ucoins[i].clone();
            ucoin.check()?;
            ucoins_balance += ucoin.amount;
        }
        if ucoins_balance != self.balance {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidAmount).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        let name_buf = self.name.as_bytes();
        buf.put_u32::<BigEndian>(name_buf.len() as u32);
        buf.put(name_buf);
        let balance_buf = self.balance.to_bytes()?;
        buf.put_u32::<BigEndian>(balance_buf.len() as u32);
        buf.put(balance_buf);
        let scoins_len = self.scoins.len();
        buf.put_u32::<BigEndian>(scoins_len as u32);
        for i in 0..scoins_len {
            let scoin_buf = self.scoins[i].to_bytes()?;
            let size = scoin_buf.len();
            buf.put_u32::<BigEndian>(size as u32);
            buf.put(scoin_buf);
        }
        let ucoins_len = self.ucoins.len();
        buf.put_u32::<BigEndian>(ucoins_len as u32);
        for i in 0..ucoins_len {
            let ucoin_buf = self.ucoins[i].to_bytes()?;
            let size = ucoin_buf.len();
            buf.put_u32::<BigEndian>(size as u32);
            buf.put(ucoin_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YWallet> {
        if buf.len() < 8 {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidLength).into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let name_len = BigEndian::read_u32(b.get(0..4).unwrap()) as usize;
        let i = 4 + name_len;
        let name = String::from_utf8_lossy(b.get(4..i).unwrap()).into();
        let balance_size = BigEndian::read_u32(b.get(i..4+i).unwrap()) as usize;
        let j = 4 + i + balance_size;
        let balance = YAmount::from_bytes(b.get(4+i..j).unwrap())?;
        let scoins_len = BigEndian::read_u32(b.get(j..4+j).unwrap()) as usize;
        let mut scoins = Vec::new();
        for k in 0..scoins_len {
            let scoin_size = BigEndian::read_u32(b.get(k..k+4).unwrap()) as usize;
            let kk = k+4+scoin_size;
            let scoin = YCoin::from_bytes(b.get(k+4..kk).unwrap())?;
            scoins.push(scoin);
        }
        let ucoins_len = BigEndian::read_u32(b.get(j..4+j).unwrap()) as usize;
        let mut ucoins = Vec::new();
        for k in 0..ucoins_len {
            let ucoin_size = BigEndian::read_u32(b.get(k..k+4).unwrap()) as usize;
            let kk = k+4+ucoin_size;
            let ucoin = YCoin::from_bytes(b.get(k+4..kk).unwrap())?;
            ucoins.push(ucoin);
        }
        let wallet = YWallet {
            name: name,
            balance: balance,
            scoins: scoins,
            ucoins: ucoins,
        };
        wallet.check()?;
        Ok(wallet)
    }
}
