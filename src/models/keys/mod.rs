use libyobicash::crypto::elliptic::keys::*;
use serde_json;
use bytes::{BufMut, BytesMut};
use store::common::*;
use models::bucket::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YKeys {
    pub sk: YSecretKey,
    pub pk: YPublicKey,
}

impl YKeys {
    pub fn new() -> YKeys {
        let sk = YSecretKey::random();
        let pk = sk.to_public();
        YKeys {
            sk: sk,
            pk: pk,
        }
    }

    pub fn check(&self) -> YHResult<()> {
        if self.pk == self.sk.to_public() {
            return Err(YHErrorKind::InvalidKey.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.sk.to_bytes());
        buf.put(self.pk.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YKeys> {
        if buf.len() != 128 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let sk = YSecretKey::from_bytes(&buf[0..64])?;
        let pk = YPublicKey::from_bytes(&buf[64..128])?;
        let keys = YKeys {
            sk: sk,
            pk: pk,
        };
        Ok(keys)
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YKeys> {
        let keys = serde_json::from_str(s)?;
        Ok(keys)
    }

    pub fn key(&self) -> YHResult<YStoreKey> {
        self.check()?;
        let key = self.pk.to_bytes();
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YKeys> {
        YKeys::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, pk: YPublicKey) -> YHResult<bool> {
        let store_buck = YBucket::Keys.to_store_buck();
        let key = pk.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Keys.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YKeys>> {
        let store_buck = YBucket::Keys.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut data = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let d = YKeys::from_value(&item.value)?;
            data.push(d);
        }
        Ok(data)
    }

    pub fn get<S: YStorage>(store: &S, pk: YPublicKey) -> YHResult<YKeys> {
        let store_buck = YBucket::Keys.to_store_buck();
        let key = pk.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YKeys::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Keys.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Keys.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
