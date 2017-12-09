use libyobicash::errors::*;

pub type YStoreBuck = Vec<u8>;

pub type YStoreKey = Vec<u8>;

pub type YStoreValue = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YStoreItem {
    pub key: YStoreKey,
    pub value: YStoreValue,
}

pub trait YStorage
    where Self: Sized
{
    type Config;

    fn create(config: Self::Config) -> YResult<Self>;

    fn open(config: Self::Config) -> YResult<Self>;

    fn close(&mut self) -> YResult<()>;

    fn reset(self) -> YResult<Self>;

    fn destroy(self) -> YResult<()>;

    fn put(&mut self, buck: &YStoreBuck, key: &YStoreKey, value: &YStoreValue) -> YResult<()>;

    fn lookup(&self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<bool>;

    fn get(&self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<YStoreItem>;

    fn count(&self, buck: &YStoreBuck) -> YResult<u64>;

    fn list(&self, buck: &YStoreBuck) -> YResult<Vec<YStoreKey>>;

    fn delete(&mut self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<()>;
}
