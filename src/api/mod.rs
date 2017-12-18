use errors::*;
use version::*;
use store::*;
use models::*;
use config::*;

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

    pub fn info() {
        unreachable!()
    }

    pub fn put_peer() {
        unreachable!()
    }

    pub fn list_peers() {
        unreachable!()
    }

    pub fn get_peer() {
        unreachable!()
    }

    pub fn delete_peer() {
        unreachable!()
    }

    pub fn cleanup_peers() {
        unreachable!()
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

    // TODO: add a method for rpc (maybe also via ipc, or smtg)
    pub fn mine() {
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
}
