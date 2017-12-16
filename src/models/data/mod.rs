use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::mac::YMACCode;
use libyobicash::data::YData as LibData;
use bytes::BufMut;
use store::common::*;
use models::bucket::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YData(pub LibData);

impl YData {
    pub fn new(data: &LibData) -> YHResult<YData> {
        data.check()?;
        Ok(YData(data.clone()))
    }

    pub fn check(&self) -> YHResult<()> {
        self.0.check()?;
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let buf = self.0.to_bytes()?;
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YData> {
        Ok(YData(LibData::from_bytes(buf)?))
    }

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut key = Vec::new();
        key.put(self.0.checksum.to_bytes());
        key.put(self.0.tag.to_bytes());
        Ok(key)
    }

    pub fn lookup<S: YStorage>(store: &S, checksum: YDigest64, tag: YMACCode) -> YHResult<bool> {
        let store_buck = YBucket::Data.to_store_buck();
        let mut key = Vec::new();
        key.put(checksum.to_bytes());
        key.put(tag.to_bytes());
        store.lookup(&store_buck, &key)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YData>> {
        let store_buck = YBucket::Data.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut data = Vec::new();        
        for key in keys {
            let d_buf = store.get(&store_buck, &key)?.value;
            let d = YData::from_bytes(&d_buf)?;
            data.push(d);
        }
        Ok(data)
    }

    pub fn get<S: YStorage>(store: &S, checksum: YDigest64, tag: YMACCode) -> YHResult<YData> {
        let store_buck = YBucket::Data.to_store_buck();
        let mut key = Vec::new();
        key.put(checksum.to_bytes());
        key.put(tag.to_bytes());
        let item = store.get(&store_buck, &key)?;
        YData::from_bytes(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Data.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.to_bytes()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Data.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
