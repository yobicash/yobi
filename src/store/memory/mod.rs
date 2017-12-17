use unqlite::{UnQLite, KV, Cursor, Direction};
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::error::Error;
use errors::*;
use store::common::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MemoryMode {
    pub path: String,
    pub read_only: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MemoryConfig {
    pub mode: MemoryMode,
}

pub struct MemoryStore {
    pub config: MemoryConfig,
    pub handle: UnQLite,
}

impl YStorage for MemoryStore {
    type Config = MemoryConfig;
        
    fn create(config: Self::Config) -> YHResult<Self> {
        Ok(MemoryStore {
            config: config,
            handle: UnQLite::create_in_memory(),
        })
    }

    fn open(config: Self::Config) -> YHResult<Self> {
        if config.mode.read_only {
            let handle = UnQLite::open_mmap(config.mode.path.as_str());
            Ok(MemoryStore {
                config: config,
                handle: handle,
            })
        } else {
            let handle = UnQLite::create_in_memory();
            Ok(MemoryStore {
                config: config,
                handle: handle,
            })
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
        let _ = self.handle;
        Ok(())
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

    fn count(&self, buck: &YStoreBuck) -> YHResult<u32> {
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
            .map_err(|err| YHErrorKind::Store(err).into())
    }
}
