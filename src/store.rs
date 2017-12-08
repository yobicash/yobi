use libyobicash::errors::*;
use unqlite::{UnQLite, KV, Cursor, Direction};
use std::marker::Sized;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::fs::remove_file;
use std::error::Error;

pub type YStoreBuck = Vec<u8>;

pub type YStoreKey = Vec<u8>;

pub type YStoreValue = Vec<u8>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YStoreItem {
    key: YStoreKey,
    value: YStoreValue,
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UnQLiteMode {
    Memory {
        path: String,
        read_only: bool,
    },
    Temporary,
    Persistent {
        path: String,
        read_only: bool,
    },
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UnQLiteConfig {
    pub mode: UnQLiteMode,
}

pub struct UnQLiteStore {
    pub config: UnQLiteConfig,
    pub handle: UnQLite,
}

impl YStorage for UnQLiteStore {
    type Config = UnQLiteConfig;
        
    fn create(config: Self::Config) -> YResult<Self> {
        match config.mode.clone() {
            UnQLiteMode::Memory { .. } => {
                Ok(UnQLiteStore {
                    config: config,
                    handle: UnQLite::create_in_memory(),
                })
            },
            UnQLiteMode::Temporary => {
                Ok(UnQLiteStore {
                    config: config,
                    handle: UnQLite::create_temp(),
                })
            },
            UnQLiteMode::Persistent{ path, .. } => {
                Ok(UnQLiteStore {
                    config: config,
                    handle: UnQLite::create(path.as_str()),
                })
            },
        }
    }

    fn open(config: Self::Config) -> YResult<Self> {
        match config.mode.clone() {
            UnQLiteMode::Memory { path, read_only } => {
                if read_only {
                    let handle = UnQLite::open_mmap(path.as_str());
                    Ok(UnQLiteStore {
                        config: config,
                        handle: handle,
                    })
                } else {
                    let handle = UnQLite::create_in_memory();
                    Ok(UnQLiteStore {
                        config: config,
                        handle: handle,
                    })
                }
            },
            UnQLiteMode::Temporary => {
                Ok(UnQLiteStore {
                    config: config,
                    handle: UnQLite::create_temp(),
                })
            },
            UnQLiteMode::Persistent{ path, read_only } => {
                if read_only {
                    let handle = UnQLite::open_readonly(path.as_str());
                    Ok(UnQLiteStore {
                        config: config,
                        handle: handle,
                    })
                } else {
                    let handle = UnQLite::create(path.as_str());
                    Ok(UnQLiteStore {
                        config: config,
                        handle: handle,
                    })
                }
            },
        }
    }

    fn close(&mut self) -> YResult<()> {
        Ok(())
    }

    fn reset(self) -> YResult<Self> {
        let config = self.config.clone();
        self.destroy()?;
        Self::create(config)
    }

    fn destroy(self) -> YResult<()> {
        match self.config.mode {
            UnQLiteMode::Memory { .. } => {
                Ok(())
            },
            UnQLiteMode::Temporary => {
                Ok(())
            },
            UnQLiteMode::Persistent{ path, read_only } => {
                if read_only {
                    let err = IOError::new(IOErrorKind::PermissionDenied, "read only store");
                    Err(YErrorKind::IO(err).into())
                } else {
                    remove_file(path.as_str())
                        .map_err(|err| YErrorKind::IO(err).into())
                }
            },
        }
    }

    fn put(&mut self, buck: &YStoreBuck, key: &YStoreKey, value: &YStoreValue) -> YResult<()> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_store(index.as_slice(), value.as_slice())
            .map_err(|err| YErrorKind::IO(IOError::new(IOErrorKind::Other, err.description())).into())
    }

    fn lookup(&self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<bool> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        Ok(self.handle.kv_contains(index.as_slice()))
    }

    fn get(&self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<YStoreItem> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_fetch(index.as_slice())
            .map(|value| YStoreItem { key: key.clone(), value: value })
            .map_err(|err| YErrorKind::IO(IOError::new(IOErrorKind::Other, err.description())).into())
    }

    fn count(&self, buck: &YStoreBuck) -> YResult<u64> {
        let mut entry = self.handle.seek(buck.as_slice(), Direction::Ge);
        if entry.is_none() {
            return Err(YErrorKind::IO(IOError::new(IOErrorKind::NotFound, "buck not found")).into());
        } else {
            let mut count = 0;
            loop {
                count += 1;
                entry = entry.unwrap().next();
                if entry.is_none() {
                    break;
                }
            }
            Ok(count)
        }
    }

    fn list(&self, buck: &YStoreBuck) -> YResult<Vec<YStoreKey>> {
        let mut entry = self.handle.seek(buck.as_slice(), Direction::Ge);
        if entry.is_none() {
            return Err(YErrorKind::IO(IOError::new(IOErrorKind::NotFound, "buck not found")).into());
        } else {
            let mut ls = Vec::new();
            loop {
                let record = entry.unwrap();
                ls.push(record.key());
                entry = record.next();
                if entry.is_none() {
                    break;
                }
            }
            Ok(ls)
        }
    }

    fn delete(&mut self, buck: &YStoreBuck, key: &YStoreKey) -> YResult<()> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_delete(index.as_slice())
            .map_err(|err| YErrorKind::IO(IOError::new(IOErrorKind::Other, err.description())).into())
    }
}
