use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA256;
use libyobicash::crypto::mac::YMACCode;
use libyobicash::crypto::key::YKey32;
use std::net::Ipv4Addr;
use errors::*;
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

    pub fn create_wallet(&mut self, name: &str) -> YHResult<()> {
        let wallet = YWallet::new(name);
        let pswd_seed = self.config.password.as_bytes();
        let key = YKey32(YSHA256::hash(&pswd_seed).0);
        wallet.create(&mut self.store.persistent, key)
    }

    pub fn list_wallets(&self, skip: u32, count: u32) -> YHResult<Vec<YWallet>> {
        let pswd_seed = self.config.password.as_bytes();
        let key = YKey32(YSHA256::hash(&pswd_seed).0);
        YWallet::list(&self.store.persistent, key, skip, count)
    }

    pub fn get_wallet(&self, name: &str) -> YHResult<YWallet> {
        let pswd_seed = self.config.password.as_bytes();
        let key = YKey32(YSHA256::hash(&pswd_seed).0);
        YWallet::get(&self.store.persistent, key, name)
    }

    pub fn list_data(&self, skip: u32, count: u32) -> YHResult<Vec<YData>> {
        YData::list(&self.store.persistent, skip, count)
    }

    pub fn list_data_by_wallet(&self, wallet_name: &str) -> YHResult<Vec<YData>> {
        let ucoins = self.list_ucoins(wallet_name)?;
        let mut data = Vec::new();
        for ucoin in ucoins {
            let d = self.get_data(ucoin.id, ucoin.tag.unwrap())?;
            data.push(d);
        }
        Ok(data)
    }

    pub fn get_data(&self, checksum: YDigest64, tag: YMACCode) -> YHResult<YData> {
        YData::get(&self.store.persistent, checksum, tag)
    }

    pub fn list_coins(&self, wallet: &str) -> YHResult<Vec<YCoin>> {
        let wallet = self.get_wallet(wallet)?;
        let mut coins = Vec::new();
        coins.extend(wallet.ucoins.clone());
        coins.extend(wallet.scoins.clone());
        Ok(coins)
    }

    pub fn list_ucoins(&self, wallet: &str) -> YHResult<Vec<YCoin>> {
        let wallet = self.get_wallet(wallet)?;
        Ok(wallet.ucoins)
    }

    pub fn list_scoins(&self, wallet: &str) -> YHResult<Vec<YCoin>> {
        let wallet = self.get_wallet(wallet)?;
        Ok(wallet.scoins)
    }

    pub fn create_transaction() {
        // NB: have to update the wallet too
        unreachable!()
    }

    pub fn create_coins_transaction() {
        // NB: have to update the wallet too
        unreachable!()
    }

    pub fn create_data_transaction() {
        // NB: have to update the wallet too
        unreachable!()
    }

    pub fn list_transactions(&self, skip: u32, count: u32) -> YHResult<Vec<YTransaction>> {
        YTransaction::list(&self.store.persistent, skip, count)
    }

    pub fn list_transactions_by_wallet(&self, wallet_name: &str) -> YHResult<Vec<YTransaction>> {
        let coins = self.list_coins(wallet_name)?;
        let mut transactions = Vec::new();
        for coin in coins {
            if coin.kind == YCoinKind::Transaction {
                let transaction = self.get_transaction(coin.id)?;
                transactions.push(transaction);
            }
        }
        Ok(transactions)
    }

    pub fn list_transaction_ancestors() {
        unreachable!()
    }

    pub fn get_transaction(&self, id: YDigest64) -> YHResult<YTransaction> {
        YTransaction::get(&self.store.persistent, id)
    }

    pub fn confirm_transaction() {
        unreachable!()
    }

    pub fn get_coinbase(&self, id: YDigest64) -> YHResult<YCoinbase> {
        YCoinbase::get(&self.store.persistent, id)
    }

    pub fn list_coinbases(&self, skip: u32, count: u32) -> YHResult<Vec<YCoinbase>> {
        YCoinbase::list(&self.store.persistent, skip, count)
    }

    pub fn list_coinbases_by_wallet(&self, wallet_name: &str) -> YHResult<Vec<YCoinbase>> {
        let coins = self.list_coins(wallet_name)?;
        let mut coinbases = Vec::new();
        for coin in coins {
            if coin.kind == YCoinKind::Coinbase {
                let coinbase = self.get_coinbase(coin.id)?;
                coinbases.push(coinbase);
            }
        }
        Ok(coinbases)
    }

    // TODO: add a method for rpc (maybe also via ipc, or smtg)
    pub fn mine() {
        // NB: have to update the wallet too
        unreachable!()
    }

    pub fn info() {
        unreachable!()
    }
}
