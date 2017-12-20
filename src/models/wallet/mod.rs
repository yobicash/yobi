use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::amount::YAmount;
use libyobicash::crypto::key::YKey32;
use libyobicash::crypto::encryption::symmetric::YSymmetricEncryption as YSE;
use serde_json;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use store::common::*;
use models::bucket::*;
use models::coin::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
        let balance_buf = self.balance.to_bytes();
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
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let name_len = BigEndian::read_u32(b.get(0..4).unwrap()) as usize;
        let i = 4 + name_len;
        let name = String::from_utf8_lossy(b.get(4..i).unwrap()).into();
        let balance_size = BigEndian::read_u32(b.get(i..4+i).unwrap()) as usize;
        let j = 4 + i + balance_size;
        let balance = YAmount::from_bytes(b.get(4+i..j).unwrap());
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

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YWallet> {
        let wallet = serde_json::from_str(s)?;
        Ok(wallet)
    }

    pub fn enc_key(&self, ekey: YKey32) -> YHResult<YStoreKey> {
        self.check()?;
        let mut key = Vec::new();
        key.put(self.name.as_bytes());
        let padding = key.len() % 16;
        for _ in 0..padding {
            key.push(0);
        }
        key = YSE::encrypt(ekey, &key)?;
        Ok(key)
    }

    pub fn enc_value(&self, ekey: YKey32) -> YHResult<YStoreValue> {
        self.check()?;
        let wallet_buf = self.to_bytes()?;
        let wallet_len = wallet_buf.len() as u32;
        let mut value = Vec::new();
        value.put_u32::<BigEndian>(wallet_len);
        value.put(wallet_buf);
        let padding = value.len() % 16;
        for _ in 0..padding {
            value.push(0);
        }
        value = YSE::encrypt(ekey, &value)?;
        Ok(value)
    }

    pub fn dec_value(ekey: YKey32, value: &YStoreValue) -> YHResult<YWallet> {
        let dec = YSE::decrypt(ekey, value)?;
        let wallet_len = BigEndian::read_u32(dec.get(0..4).unwrap());
        let wallet_buf = dec.get(4..wallet_len as usize).unwrap();
        YWallet::from_bytes(wallet_buf)
    }

    pub fn lookup<S: YStorage>(store: &S, ekey: YKey32, name: &str) -> YHResult<bool> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let mut key = Vec::new();
        key.put(name.as_bytes());
        let padding = key.len() % 16;
        for _ in 0..padding {
            key.push(0);
        }
        key = YSE::encrypt(ekey, &key)?;
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Wallets.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, ekey: YKey32, skip: u32, count: u32) -> YHResult<Vec<YWallet>> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut wallets = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let wallet = YWallet::dec_value(ekey, &item.value)?;
            wallets.push(wallet);
        }
        Ok(wallets)
    }

    pub fn select_coins_no_data<S: YStorage>(store: &S, ekey: YKey32, name: &str, amount: YAmount) -> YHResult<Vec<YCoin>> {
        let wallet = YWallet::get(store, ekey, name)?;
        if wallet.balance < amount {
            return Err(YHErrorKind::NotEnoughFunds.into());    
        }
        let mut coins = Vec::new();
        let mut tot_amount = YAmount::zero();
        for ucoin in wallet.ucoins {
            if !ucoin.has_data {
                coins.push(ucoin.clone());
                tot_amount += ucoin.amount;
                if tot_amount >= amount {
                    break;
                }
            }
        }
        if tot_amount >= amount {
            return Ok(coins);
        } else {
            return Err(YHErrorKind::NotEnoughFunds.into());
        }
    }

    pub fn select_coins<S: YStorage>(store: &S, ekey: YKey32, name: &str, amount: YAmount) -> YHResult<Vec<YCoin>> {
        let wallet = YWallet::get(store, ekey, name)?;
        if wallet.balance < amount {
            return Err(YHErrorKind::NotEnoughFunds.into());    
        }
        if wallet.balance == amount {
            return Ok(wallet.ucoins);
        }
        let mut coins = Vec::new();
        let mut tot_amount = YAmount::zero();
        for ucoin in wallet.ucoins {
            coins.push(ucoin.clone());
            tot_amount += ucoin.amount;
            if tot_amount >= amount {
                break;
            }
        }
        if tot_amount >= amount {
            return Ok(coins);
        } else {
            return Err(YHErrorKind::NotEnoughFunds.into());
        }
    }

    pub fn get<S: YStorage>(store: &S, ekey: YKey32, name: &str) -> YHResult<YWallet> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let mut key = Vec::new();
        key.put(name.as_bytes());
        let padding = key.len() % 16;
        for _ in 0..padding {
            key.push(0);
        }
        key = YSE::encrypt(ekey, &key)?;
        let item = store.get(&store_buck, &key)?;
        YWallet::dec_value(ekey, &item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S, ekey: YKey32) -> YHResult<()> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let key = self.enc_key(ekey)?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.enc_value(ekey)?;
        store.put(&store_buck, &key, &value)
    }

    pub fn update<S: YStorage>(&self, store: &mut S, ekey: YKey32) -> YHResult<()> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let key = self.enc_key(ekey)?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        let value = self.enc_value(ekey)?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S, ekey: YKey32) -> YHResult<()> {
        let store_buck = YBucket::Wallets.to_store_buck();
        let key = self.enc_key(ekey)?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
