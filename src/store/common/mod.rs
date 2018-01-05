use errors::*;

#[derive(Debug)]
pub enum YStorageKind {
    Persistent=0,
    Memory=1,
}

impl Default for YStorageKind {
    fn default() -> YStorageKind {
        YStorageKind::Persistent
    }
}

#[derive(Debug)]
pub enum YStorageMode {
    Full=0,
    Light=1,
}

impl Default for YStorageMode {
    fn default() -> YStorageMode {
        YStorageMode::Full
    }
}

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

    fn create(config: Self::Config) -> YHResult<Self>;

    fn open(config: Self::Config) -> YHResult<Self>;

    fn close(&mut self) -> YHResult<()>;

    fn reset(&mut self) -> YHResult<Self>;

    fn destroy(&mut self) -> YHResult<()>;

    fn put(&mut self, buck: &YStoreBuck, key: &YStoreKey, value: &YStoreValue) -> YHResult<()>;

    fn lookup(&self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<bool>;

    fn get(&self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<YStoreItem>;

    fn count(&self, buck: &YStoreBuck) -> YHResult<u32>;

    fn list(&self, buck: &YStoreBuck, skip: u32, count: u32) -> YHResult<Vec<YStoreKey>>;

    fn list_reverse(&self, buck: &YStoreBuck, skip: u32, count: u32) -> YHResult<Vec<YStoreKey>>;

    fn delete(&mut self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<()>;
}
