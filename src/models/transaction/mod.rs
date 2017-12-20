use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::transaction::YTransaction as LibTransaction;
use serde_json;
use store::common::*;
use models::bucket::*;
use models::coinbase::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YTransaction(pub LibTransaction);

impl YTransaction {
    pub fn new(transaction: &LibTransaction) -> YHResult<YTransaction> {
        transaction.check()?;
        let tx = transaction.clone().drop_all();
        tx.check()?;
        Ok(YTransaction(tx.clone()))
    }

    pub fn internal(&self) -> LibTransaction {
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

    pub fn from_bytes(buf: &[u8]) -> YHResult<YTransaction> {
        Ok(YTransaction(LibTransaction::from_bytes(buf)?))
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YTransaction> {
        let tx = serde_json::from_str(s)?;
        Ok(tx)
    }

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let key = self.0.id.to_bytes();
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YTransaction> {
        YTransaction::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64) -> YHResult<bool> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Transactions.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YTransaction>> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut transactions = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let cb = YTransaction::from_value(&item.value)?;
            transactions.push(cb);
        }
        Ok(transactions)
    }

    pub fn list_ancestors<S: YStorage>(store: &S, id: YDigest64, level: u32)
            -> YHResult<(Vec<YTransaction>, Vec<YCoinbase>)> {
        let start_tx = YTransaction::get(store, id)?.internal();
        if start_tx.outputs[0].height < level + 1 {
            return Err(YHErrorKind::InvalidLevel.into());
        }
        let mut inputs = YTransaction::get(store, id)?.internal().inputs;
        let mut ancestor_tx_ids = Vec::new();
        let mut ancestor_txs = Vec::new();
        let mut ancestor_cb_ids = Vec::new();
        let mut ancestor_cbs = Vec::new();
        let mut height = level;
        while height > 0 {
            let mut prev_inputs = Vec::new();
            for input in inputs {
                let id = input.id;
                if input.height != 0 {
                    if !ancestor_tx_ids.contains(&id) {
                        ancestor_tx_ids.push(id);
                        let tx = YTransaction::get(store, id)?;
                        prev_inputs.extend(tx.internal().inputs.clone());
                        ancestor_txs.push(tx);
                    }
                } else {
                    if !ancestor_cb_ids.contains(&id) {
                        ancestor_cb_ids.push(id);
                        ancestor_cbs.push(YCoinbase::get(store, id)?);
                    }
                }
            }
            inputs = prev_inputs.clone();
            height -= 1;
        }
        Ok((ancestor_txs, ancestor_cbs))
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64) -> YHResult<YTransaction> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YTransaction::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
