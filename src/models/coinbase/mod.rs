use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::coinbase::YCoinbase as LibCoinbase;
use serde_json;
use store::common::*;
use models::bucket::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YCoinbase(pub LibCoinbase);

impl YCoinbase {
    pub fn new(coinbase: &LibCoinbase) -> YHResult<YCoinbase> {
        coinbase.check()?;
        Ok(YCoinbase(coinbase.clone()))
    }

    pub fn internal(&self) -> LibCoinbase {
        self.0.clone()
    }

    pub fn check(&self) -> YHResult<()> {
        self.0.check()?;
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let buf = self.0.to_bytes()?;
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YCoinbase> {
        Ok(YCoinbase(LibCoinbase::from_bytes(buf)?))
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YCoinbase> {
        let coinbase = serde_json::from_str(s)?;
        Ok(coinbase)
    }

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let key = self.0.id.to_bytes();
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YCoinbase> {
        YCoinbase::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64) -> YHResult<bool> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = id.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YCoinbase>> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut coinbases = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let cb = YCoinbase::from_value(&item.value)?;
            coinbases.push(cb);
        }
        Ok(coinbases)
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64) -> YHResult<YCoinbase> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = id.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YCoinbase::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
