use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::key::YKey32;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::coinbase::YCoinbase as LibCoinbase;
use serde_json;
use std::cmp::min;
use store::common::*;
use models::bucket::*;
use models::transaction::*;
use models::coin::*;
use models::wallet::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YCoinbase(pub LibCoinbase);

impl YCoinbase {
    pub fn new(coinbase: &LibCoinbase) -> YHResult<YCoinbase> {
        coinbase.check()?;
        Ok(YCoinbase(coinbase.clone()))
    }

    pub fn internal(&self) -> LibCoinbase {
        self.0.clone()
    }

    pub fn check(&self) -> YHResult<()> {
        self.0.check()?;
        Ok(())
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let buf = self.0.to_bytes()?;
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YCoinbase> {
        Ok(YCoinbase(LibCoinbase::from_bytes(buf)?))
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YCoinbase> {
        let coinbase = serde_json::from_str(s)?;
        Ok(coinbase)
    }

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let key = self.0.id.to_bytes();
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YCoinbase> {
        YCoinbase::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64) -> YHResult<bool> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = id.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YCoinbase>> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut coinbases = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let cb = YCoinbase::from_value(&item.value)?;
            coinbases.push(cb);
        }
        Ok(coinbases)
    }

    pub fn mine<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, id: YDigest64, incr: u32, fee_pk: YPublicKey) -> YHResult<(YCoinbase, u32)> {
        let tx = YTransaction::get(store, id)?;
        let diff = LibCoinbase::difficulty(tx.internal().outputs[0].height);
        let (txs, cbs) = tx.list_ancestors(store)?;
        let txs_len = txs.len();
        let cbs_len = cbs.len();
        let txs_chunks_len = min(txs_len as u32, diff);
        let cbs_chunks_len = min(cbs_len as u32, diff);
        let txs_indexes = YRandom::u32_sample(0, txs_len as u32, txs_chunks_len as u32);
        let cbs_indexes = YRandom::u32_sample(0, cbs_len as u32, cbs_chunks_len as u32);
        let mut chunks = Vec::new();

        for tx_idx in txs_indexes {
            let buf = txs[tx_idx as usize].internal().to_bytes()?;
            let buf_len = buf.len();
            let byte_idx = YRandom::u32_range(0, buf_len as u32);
            chunks.push(buf[byte_idx as usize]);
        }

        for cb_idx in cbs_indexes {
            let buf = cbs[cb_idx as usize].internal().to_bytes()?;
            let buf_len = buf.len();
            let byte_idx = YRandom::u32_range(0, buf_len as u32);
            chunks.push(buf[byte_idx as usize]);
        }

        let miner_sk = YSecretKey::random();
        let recipient_sk = YSecretKey::random();
        let recipient_pk = recipient_sk.to_public();
        
        let (_cb, tries) = LibCoinbase::mine(id, diff,
                                             &chunks, incr,
                                             miner_sk, recipient_pk,
                                             fee_pk)?;
        let cb = YCoinbase(_cb.clone());
        cb.create(store)?;

        let date = _cb.time;
        let kind = YCoinKind::Coinbase;
        let id = _cb.id;
        let idx = 0u32;
        let height = 0u32;
        let has_data = false;
        let tag = None;
        let amount = _cb.outputs[idx as usize].amount.clone();
        let coin = YCoin::new(date, miner_sk,
                              kind, id, idx,
                              height, has_data,
                              tag, &amount)?;
        
        let mut wallet = YWallet::get(store, key, wallet_name)?;
        wallet.balance += amount;
        wallet.ucoins.push(coin);
        wallet.update(store, key)?;

        Ok((cb, tries))
    }

    pub fn mine_genesys<S: YStorage>(store: &mut S,
                                     key: YKey32,
                                     wallet_name: &str,
                                     incr: u32, fee_pk: YPublicKey)
            -> YHResult<((YCoinbase, YTransaction), u32)> {
        let diff = 3;

        let miner_sk = YSecretKey::random();
        let recipient_sk = YSecretKey::random();
        let recipient_pk = recipient_sk.to_public();

        let chunks = YRandom::bytes(diff);
        let ((_cb, _tx), tries) = LibCoinbase::mine_genesys(incr, &chunks,
                                                            miner_sk, recipient_pk,
                                                            fee_pk)?;

        let tx = YTransaction(_tx.clone());
        tx.create(store)?;

        let cb = YCoinbase(_cb.clone());
        cb.create(store)?;

        let date = _cb.time;
        let kind = YCoinKind::Coinbase;
        let id = _cb.id;
        let idx = 0u32;
        let height = 0u32;
        let has_data = false;
        let tag = None;
        let amount = _cb.outputs[idx as usize].amount.clone();
        let coin = YCoin::new(date, miner_sk,
                              kind, id, idx,
                              height, has_data,
                              tag, &amount)?;
        
        let mut wallet = YWallet::get(store, key, wallet_name)?;
        wallet.balance += amount;
        wallet.ucoins.push(coin);
        wallet.update(store, key)?;

        Ok(((cb, tx), tries))
    }

    pub fn confirm<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, id: YDigest64, incr: u32, fee_pk: YPublicKey) -> YHResult<(bool, Option<(YCoinbase, YTransaction)>)> {
        match YCoinbase::lookup(store, id) {
            Ok(true) => {
                let (genesys, _) = YCoinbase::mine_genesys(store, key, wallet_name, incr, fee_pk)?;
                Ok((true, Some(genesys)))
            },
            Ok(false) => {
                Ok((false, None))
            },
            Err(e) => Err(e),
        }
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64) -> YHResult<YCoinbase> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = id.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YCoinbase::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Coinbases.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
