use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::transaction::YTransaction as LibTransaction;
use store::common::*;
use models::bucket::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YTransaction(pub LibTransaction);

impl YTransaction {
    pub fn new(transaction: &LibTransaction) -> YHResult<YTransaction> {
        transaction.check()?;
        let tx = transaction.clone().drop_all();
        tx.check()?;
        Ok(YTransaction(tx.clone()))
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

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let key = self.0.id.to_bytes();
        Ok(key)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64) -> YHResult<bool> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YTransaction>> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut transactions = Vec::new();        
        for key in keys {
            let cb_buf = store.get(&store_buck, &key)?.value;
            let cb = YTransaction::from_bytes(&cb_buf)?;
            transactions.push(cb);
        }
        Ok(transactions)
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64) -> YHResult<YTransaction> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YTransaction::from_bytes(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.to_bytes()?;
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
