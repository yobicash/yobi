use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::utxo::YUTXO as LibUTXO;
use bytes::{BufMut, BigEndian};
use serde_json;
use store::common::*;
use models::bucket::*;
use models::transaction::*;
use models::coinbase::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YUTXO(pub LibUTXO);

impl YUTXO {
    pub fn new(utxo: &LibUTXO) -> YUTXO {
        YUTXO(utxo.clone())
    }

    pub fn internal(&self) -> LibUTXO {
        self.0.clone()
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let buf = self.0.to_bytes()?;
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YUTXO> {
        Ok(YUTXO(LibUTXO::from_bytes(buf)?))
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YUTXO> {
        let utxo = serde_json::from_str(s)?;
        Ok(utxo)
    }

    pub fn key(&self) -> YHResult<YStoreKey> {
        let mut key = Vec::new();
        key.put(self.0.id.to_bytes());
        key.put_u32::<BigEndian>(self.0.idx);
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YUTXO> {
        YUTXO::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64, idx: u32) -> YHResult<bool> {
        let store_buck = YBucket::UTXO.to_store_buck();
        let mut key = Vec::new();
        key.put(id.to_bytes());
        key.put_u32::<BigEndian>(idx);
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::UTXO.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YUTXO>> {
        let store_buck = YBucket::UTXO.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut utxos = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let utxo = YUTXO::from_value(&item.value)?;
            utxos.push(utxo);
        }
        Ok(utxos)
    }

    pub fn list_by_tx<S: YStorage>(store: &S, id: YDigest64) -> YHResult<Vec<YUTXO>> {
        let tx = YTransaction::get(store, id)?;
        let outputs_len = tx.internal().outputs.len() as u32;
        let mut utxos = Vec::new();
        let store_buck = YBucket::UTXO.to_store_buck();
        for idx in 0..outputs_len {
            let mut key = Vec::new();
            key.put(id.to_bytes());
            key.put_u32::<BigEndian>(idx);
            let item = store.get(&store_buck, &key)?;
            let utxo = YUTXO::from_value(&item.value)?;
            utxos.push(utxo);
        }
        Ok(utxos)
    }

    pub fn list_by_cb<S: YStorage>(store: &S, id: YDigest64) -> YHResult<Vec<YUTXO>> {
        let cb = YCoinbase::get(store, id)?;
        let outputs_len = cb.internal().outputs.len() as u32;
        let mut ucbos = Vec::new();
        let store_buck = YBucket::UTXO.to_store_buck();
        for idx in 0..outputs_len {
            let mut key = Vec::new();
            key.put(id.to_bytes());
            key.put_u32::<BigEndian>(idx);
            let item = store.get(&store_buck, &key)?;
            let ucbo = YUTXO::from_value(&item.value)?;
            ucbos.push(ucbo);
        }
        Ok(ucbos)
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64, idx: u32) -> YHResult<YUTXO> {
        let store_buck = YBucket::UTXO.to_store_buck();
        let mut key = Vec::new();
        key.put(id.to_bytes());
        key.put_u32::<BigEndian>(idx);
        let item = store.get(&store_buck, &key)?;
        YUTXO::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::UTXO.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::UTXO.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
