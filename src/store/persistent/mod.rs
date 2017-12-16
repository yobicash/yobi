use unqlite::{UnQLite, KV, Cursor, Direction};
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::fs::remove_file;
use std::error::Error;
use errors::*;
use store::common::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DbMode {
    Temporary,
    Persistent {
        path: String,
        read_only: bool,
    },
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DbConfig {
    pub mode: DbMode,
}

pub struct DbStore {
    pub config: DbConfig,
    pub handle: UnQLite,
}

impl YStorage for DbStore {
    type Config = DbConfig;
        
    fn create(config: Self::Config) -> YHResult<Self> {
        match config.mode.clone() {
            DbMode::Temporary => {
                Ok(DbStore {
                    config: config,
                    handle: UnQLite::create_temp(),
                })
            },
            DbMode::Persistent{ path, .. } => {
                Ok(DbStore {
                    config: config,
                    handle: UnQLite::create(path.as_str()),
                })
            },
        }
    }

    fn open(config: Self::Config) -> YHResult<Self> {
        match config.mode.clone() {
            DbMode::Temporary => {
                Ok(DbStore {
                    config: config,
                    handle: UnQLite::create_temp(),
                })
            },
            DbMode::Persistent{ path, read_only } => {
                if read_only {
                    let handle = UnQLite::open_readonly(path.as_str());
                    Ok(DbStore {
                        config: config,
                        handle: handle,
                    })
                } else {
                    let handle = UnQLite::create(path.as_str());
                    Ok(DbStore {
                        config: config,
                        handle: handle,
                    })
                }
            },
        }
    }

    fn close(&mut self) -> YHResult<()> {
        Ok(())
    }

    fn reset(self) -> YHResult<Self> {
        let config = self.config.clone();
        self.destroy()?;
        Self::create(config)
    }

    fn destroy(self) -> YHResult<()> {
        match self.config.mode {
            DbMode::Temporary => {
                Ok(())
            },
            DbMode::Persistent{ path, read_only } => {
                if read_only {
                    let err = IOError::new(IOErrorKind::PermissionDenied, "read only store");
                    Err(YHErrorKind::IO(err).into())
                } else {
                    remove_file(path.as_str())
                        .map_err(|err| YHErrorKind::IO(err).into())
                }
            },
        }
    }

    fn put(&mut self, buck: &YStoreBuck, key: &YStoreKey, value: &YStoreValue) -> YHResult<()> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_store(index.as_slice(), value.as_slice())
            .map_err(|err| YHErrorKind::IO(IOError::new(IOErrorKind::Other, err.description())).into())
    }

    fn lookup(&self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<bool> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        Ok(self.handle.kv_contains(index.as_slice()))
    }

    fn get(&self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<YStoreItem> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_fetch(index.as_slice())
            .map(|value| YStoreItem { key: key.clone(), value: value })
            .map_err(|err| YHErrorKind::IO(IOError::new(IOErrorKind::Other, err.description())).into())
    }

    fn count(&self, buck: &YStoreBuck) -> YHResult<u64> {
        let mut entry = self.handle.seek(buck.as_slice(), Direction::Ge);
        if entry.is_none() {
            return Err(YHErrorKind::IO(IOError::new(IOErrorKind::NotFound, "buck not found")).into());
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

    fn list(&self, buck: &YStoreBuck, skip: u32, count: u32) -> YHResult<Vec<YStoreKey>> {
        let mut entry = self.handle.seek(buck.as_slice(), Direction::Ge);
        if entry.is_none() {
            return Err(YHErrorKind::IO(IOError::new(IOErrorKind::NotFound, "buck not found")).into());
        } else {
            let mut _skip = skip;
            let mut _count = count;
            let mut ls = Vec::new();
            loop {
                if _skip == 0 {
                    break;
                }
                if count != 0 {
                    let record = entry.unwrap();
                    ls.push(record.key());
                    entry = record.next();
                    _count -= 1;
                    _skip -= 1;
                } else {
                    break;
                }
                if entry.is_none() {
                    break;
                }
            }
            Ok(ls)
        }
    }

    fn list_reverse(&self, buck: &YStoreBuck, skip: u32, count: u32) -> YHResult<Vec<YStoreKey>> {
        let mut entry = self.handle.seek(buck.as_slice(), Direction::Le);
        if entry.is_none() {
            return Err(YHErrorKind::IO(IOError::new(IOErrorKind::NotFound, "buck not found")).into());
        } else {
            let mut _skip = skip;
            let mut _count = count;
            let mut ls = Vec::new();
            loop {
                if _skip == 0 {
                    break;
                }
                if count != 0 {
                    let record = entry.unwrap();
                    ls.push(record.key());
                    entry = record.next();
                    _count -= 1;
                    _skip -= 1;
                } else {
                    break;
                }
                if entry.is_none() {
                    break;
                }
            }
            Ok(ls)
        }
    }

    fn delete(&mut self, buck: &YStoreBuck, key: &YStoreKey) -> YHResult<()> {
        let mut index = Vec::new();
        index.extend(buck.iter().cloned());
        index.extend(key.iter().cloned());
        self.handle.kv_delete(index.as_slice())
            .map_err(|err| YHErrorKind::Db(err).into())
    }
}
