use libyobicash::utils::time::YTime;
use std::net::Ipv4Addr;
use errors::*;
use version::*;
use store::*;
use network::host::YHost;
use config::*;
use models::*;

pub struct YAPIStore<M, P: YStorage> {
    pub memory: M,
    pub persistent: P,
}

pub struct YAPI<M, P: YStorage> {
    pub config: YConfig,
    pub store: YAPIStore<M, P>,
}

impl YAPI<YMemoryStore, YPersistentStore> {
    pub fn new(home_dir: Option<String>, read_only: bool) -> YHResult<YAPI<YMemoryStore, YPersistentStore>> {
        let config = YConfig::read(home_dir)?;
        let mem_config = YMemoryConfig {
            mode: YMemoryMode {
                path: config.db_path.clone(),
                read_only: read_only,
            }, 
        };
        let mem_store = YMemoryStore::open(mem_config)?;
        let per_config = YPersistentConfig {
            mode: YPersistentMode::Persistent {
                path: config.db_path.clone(),
                read_only: read_only,
            },
        };
        let per_store = YPersistentStore::open(per_config)?;
        let store = YAPIStore {
            memory: mem_store,
            persistent: per_store,
        };
        let api = YAPI::<YMemoryStore, YPersistentStore> {
            config: config,
            store: store,
        };
        Ok(api)
    }

    pub fn new_temporary(home_dir: Option<String>) -> YHResult<YAPI<YMemoryStore, YPersistentStore>> {
        let config = YConfig::read(home_dir)?;
        let mem_config = YMemoryConfig {
            mode: YMemoryMode {
                path: config.db_path.clone(),
                read_only: false,
            }, 
        };
        let mem_store = YMemoryStore::create(mem_config)?;
        let per_config = YPersistentConfig {
            mode: YPersistentMode::Temporary,
        };
        let per_store = YPersistentStore::create(per_config)?;
        let store = YAPIStore {
            memory: mem_store,
            persistent: per_store,
        };
        let api = YAPI::<YMemoryStore, YPersistentStore> {
            config: config,
            store: store,
        };
        Ok(api)
    }

    pub fn close_store(&mut self) -> YHResult<()> {
        self.store.memory.close()?;
        self.store.persistent.close()?;
        Ok(())
    }

    pub fn reset_store(&mut self) -> YHResult<()> {
        self.store.memory = self.store.memory.reset()?;
        self.store.persistent = self.store.persistent.reset()?;
        Ok(())
    }

    pub fn destroy_store(&mut self) -> YHResult<()> {
        self.store.memory.destroy()?;
        self.store.persistent.destroy()?;
        Ok(())
    }

    pub fn put_peer(&mut self, host: YHost) -> YHResult<()> {
        let peer = YPeer::new(host);
        if !YPeer::lookup_by_ip(&self.store.persistent, host.address)? {
            peer.create(&mut self.store.persistent)?;
        } else {
            peer.update(&mut self.store.persistent)?;
        }
        Ok(())
    }

    pub fn list_peers(&self, skip: u32, count: u32) -> YHResult<Vec<YPeer>> {
        YPeer::list_by_ip(&self.store.persistent, skip, count)
    }

    pub fn get_peer(&self, ip: Ipv4Addr) -> YHResult<YPeer> {
        YPeer::get(&self.store.persistent, ip)
    }

    pub fn delete_peer(&mut self, host: YHost) -> YHResult<()> {
        let peer = YPeer::new(host);
        peer.delete(&mut self.store.persistent)
    }

    pub fn cleanup_peers(&mut self, limit_time: YTime) -> YHResult<()> {
        let count = YPeer::count_by_ip(&self.store.persistent)?;
        for peer in  self.list_peers(0, count)? {
            if peer.last_time < limit_time {
                peer.delete(&mut self.store.persistent)?;
            }
        }
        Ok(())
    }

    pub fn create_wallet() {
        unreachable!()
    }

    pub fn list_wallets() {
        unreachable!()
    }

    pub fn get_wallet() {
        unreachable!()
    }

    pub fn put_data() {
        unreachable!()
    }

    pub fn list_data() {
        unreachable!()
    }

    pub fn get_data() {
        unreachable!()
    }

    pub fn list_coins() {
        unreachable!()
    }

    pub fn list_ucoins() {
        unreachable!()
    }

    pub fn list_scoins() {
        unreachable!()
    }

    pub fn create_tx() {
        unreachable!()
    }

    pub fn create_coins_tx() {
        unreachable!()
    }

    pub fn create_data_tx() {
        unreachable!()
    }

    pub fn list_txs() {
        unreachable!()
    }

    pub fn list_tx_ancestors() {
        unreachable!()
    }

    pub fn get_tx() {
        unreachable!()
    }

    pub fn confirm_tx() {
        unreachable!()
    }

    pub fn get_coinbase() {
        unreachable!()
    }

    pub fn list_coinbases() {
        unreachable!()
    }

    // TODO: add a method for rpc (maybe also via ipc, or smtg)
    pub fn mine() {
        unreachable!()
    }

    pub fn info() {
        unreachable!()
    }
}
