use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::key::YKey32;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::amount::*;
use libyobicash::utxo::YUTXO as LibUTXO;
use libyobicash::transaction::YTransaction as LibTransaction;
use serde_json;
use store::common::*;
use models::bucket::*;
use models::data::*;
use models::coinbase::*;
use models::coin::*;
use models::utxo::*;
use models::wallet::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YTransaction(pub LibTransaction);

impl YTransaction {
    pub fn new(transaction: &LibTransaction) -> YHResult<YTransaction> {
        transaction.check()?;
        let tx = transaction.clone().drop_all();
        tx.check()?;
        Ok(YTransaction(tx.clone()))
    }

    pub fn internal(&self) -> LibTransaction {
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

    pub fn from_bytes(buf: &[u8]) -> YHResult<YTransaction> {
        Ok(YTransaction(LibTransaction::from_bytes(buf)?))
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YTransaction> {
        let tx = serde_json::from_str(s)?;
        Ok(tx)
    }

    pub fn key(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let key = self.0.id.to_bytes();
        Ok(key)
    }

    pub fn value(&self) -> YHResult<YStoreValue> {
        self.to_bytes()
    }

    pub fn from_value(value: &YStoreValue) -> YHResult<YTransaction> {
        YTransaction::from_bytes(value)
    }

    pub fn lookup<S: YStorage>(store: &S, id: YDigest64) -> YHResult<bool> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        store.lookup(&store_buck, &key)
    }

    pub fn count<S: YStorage>(store: &S) -> YHResult<u32> {
        let store_buck = YBucket::Transactions.to_store_buck();
        store.count(&store_buck)
    }

    pub fn list<S: YStorage>(store: &S, skip: u32, count: u32) -> YHResult<Vec<YTransaction>> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let keys = store.list(&store_buck, skip, count)?;
        let mut transactions = Vec::new();        
        for key in keys {
            let item = store.get(&store_buck, &key)?;
            let cb = YTransaction::from_value(&item.value)?;
            transactions.push(cb);
        }
        Ok(transactions)
    }

    pub fn list_ancestors<S: YStorage>(&self, store: &S) -> YHResult<(Vec<YTransaction>, Vec<YCoinbase>)> {
        let start_tx = self.internal();
        let mut height = start_tx.outputs[0].height;
        let mut inputs = start_tx.inputs;
        let mut ancestor_tx_ids = Vec::new();
        let mut ancestor_txs = Vec::new();
        let mut ancestor_cb_ids = Vec::new();
        let mut ancestor_cbs = Vec::new();
        while height > 0 {
            let mut prev_inputs = Vec::new();
            for input in inputs {
                let id = input.id;
                if input.height != 0 {
                    if !ancestor_tx_ids.contains(&id) {
                        ancestor_tx_ids.push(id);
                        let tx = YTransaction::get(store, id)?;
                        prev_inputs.extend(tx.internal().inputs.clone());
                        ancestor_txs.push(tx);
                    }
                } else {
                    if !ancestor_cb_ids.contains(&id) {
                        ancestor_cb_ids.push(id);
                        ancestor_cbs.push(YCoinbase::get(store, id)?);
                    }
                }
            }
            inputs = prev_inputs.clone();
            height -= 1;
        }
        Ok((ancestor_txs, ancestor_cbs))
    }

    pub fn count_ancestors<S: YStorage>(store: &S, id: YDigest64)
            -> YHResult<(u32, u32)> {
        let start_tx = YTransaction::get(store, id)?.internal();
        let mut height = start_tx.outputs[0].height;
        let mut inputs = YTransaction::get(store, id)?.internal().inputs;
        let mut ancestor_tx_ids = Vec::new();
        let mut ancestor_txs_count = 0;
        let mut ancestor_cb_ids = Vec::new();
        let mut ancestor_cbs_count = 0;
        while height > 0 {
            let mut prev_inputs = Vec::new();
            for input in inputs {
                let id = input.id;
                if input.height != 0 {
                    if !ancestor_tx_ids.contains(&id) {
                        ancestor_tx_ids.push(id);
                        let tx = YTransaction::get(store, id)?;
                        prev_inputs.extend(tx.internal().inputs.clone());
                        ancestor_txs_count += 1;
                    }
                } else {
                    if !ancestor_cb_ids.contains(&id) {
                        ancestor_cb_ids.push(id);
                        ancestor_cbs_count += 1;
                    }
                }
            }
            inputs = prev_inputs.clone();
            height -= 1;
        }
        Ok((ancestor_txs_count, ancestor_cbs_count))
    }

    pub fn confirm<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, id: YDigest64, incr: u32, fee_pk: YPublicKey)
            -> YHResult<(bool, Option<YCoinbase>)> {
        match YTransaction::get(store, id) {
            Ok(_) => {
                let (cb, _) = YCoinbase::mine(store, key, wallet_name, id, incr, fee_pk)?;
                Ok((true, Some(cb)))
            },
            Err(YHError(YHErrorKind::NotFound, _)) => {
                Ok((false, None))
            },
            Err(err) => Err(err),
        }
    }

    pub fn get<S: YStorage>(store: &S, id: YDigest64) -> YHResult<YTransaction> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = id.to_bytes();
        let item = store.get(&store_buck, &key)?;
        YTransaction::from_value(&item.value)
    }

    pub fn create<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = self.key()?;
        if store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::AlreadyFound.into());
        }
        let value = self.value()?;
        store.put(&store_buck, &key, &value)
    }

    pub fn create_raw<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, raw: &str, _sks: &Vec<YSecretKey>) -> YHResult<YTransaction> {
        let _tx = LibTransaction::from_hex(raw)?;
        let date = _tx.time.clone();
        let kind = YCoinKind::Transaction;
        let id = _tx.id;
        let inputs = _tx.inputs.clone();
        let outputs = _tx.outputs.clone();
        let height = outputs[0].height;

        let mut sks = _sks.clone();
        sks.dedup();
            
        if sks.len() != outputs.len() {
            return Err(YHErrorKind::InvalidLength.into());
        }

        let mut wallet = YWallet::get(store, key, wallet_name)?;

        for input in inputs {
            let id = input.id;
            let idx = input.idx;
            let height = input.height;

            let ucoins_len = wallet.ucoins.len();

            if height != 0 {
                let tx = YTransaction::get(store, id)?.internal();
                let date = tx.time.clone();
                let kind = YCoinKind::Transaction;
                let mut found = false;

                for i in 0..ucoins_len {
                    let ucoin = wallet.ucoins[i].clone();
                    if ucoin.date == date &&
                        ucoin.kind == kind &&
                        ucoin.id == id &&
                        ucoin.idx == idx &&
                        ucoin.height == height {
                        wallet.ucoins.remove(i);
                        wallet.scoins.push(ucoin);
                        found = true;
                    }
                }
                if !found {
                    return Err(YHErrorKind::NotFound.into());
                }
            } else {
                let cb = YCoinbase::get(store, id)?.internal();
                let date = cb.time.clone();
                let kind = YCoinKind::Coinbase;
                let mut found = false;

                for i in 0..ucoins_len {
                    let ucoin = wallet.ucoins[i].clone();
                    if ucoin.date == date &&
                        ucoin.kind == kind &&
                        ucoin.id == id &&
                        ucoin.idx == idx &&
                        ucoin.height == height {
                        wallet.ucoins.remove(i);
                        wallet.scoins.push(ucoin);
                        found = true;
                    }
                }
                if !found {
                    return Err(YHErrorKind::NotFound.into());
                }
            }
        }
       
        for idx in 0..outputs.len() {
            let output = outputs[idx].clone();
            let has_data = output.data.is_some();
            let tag = if !has_data {
                None
            } else {
                Some(output.data.clone().unwrap().tag)
            };
            let amount = output.amount.clone();
            let coin = YCoin {
                date: date.clone(),
                sk: sks[idx],
                kind: kind,
                id: id,
                idx: idx as u32,
                height: height,
                has_data: has_data,
                tag: tag,
                amount: amount,
            };
            wallet.ucoins.push(coin);
            if has_data {
                let _data = output.clone().data.unwrap();
                let data = YData::new(&_data)?;
                data.create(store)?;
            }
            let _utxo = LibUTXO::from_output(&output, id, idx as u32);
            YUTXO::new(&_utxo).create(store)?;
        }

        let tx = YTransaction(_tx);
        tx.create(store)?;

        wallet.update(store, key)?;

        Ok(tx)
    }

    pub fn create_coins<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, to: YPublicKey, amount: YAmount, keep_data: bool) -> YHResult<YTransaction> {
        let coins_sk = YSecretKey::random();
        let change_sk = YSecretKey::random();
        let change_pk = change_sk.to_public();
        
        let mut wallet = YWallet::get(store, key, wallet_name)?;
        
        let ucoins = if keep_data {
            wallet.select_coins_no_data(amount.clone())?
        } else {
            wallet.select_coins(amount.clone())?
        };
       
        let mut sks = Vec::new();
        let mut xs = Vec::new();
        for ucoin in ucoins.clone() {
            sks.push(ucoin.sk);
            xs.push(ucoin.sk.sk);
        }
        
        let mut utxos = Vec::new();

        for ucoin in ucoins.clone() {
            let id = ucoin.id;
            let idx = ucoin.idx;
            let height = ucoin.height;
            let recipient = ucoin.sk.to_public();
            let amount = ucoin.amount;
            let utxo = LibUTXO::new(id, idx, height, recipient, amount);
            utxos.push(utxo);
        }

        let _tx = LibTransaction::new_coins(&coins_sk, &change_sk,
                                            &to, &change_pk, amount,
                                            &utxos, &xs,
                                            None, None)?;

        // NB: change it if the coin selection
        //     is improved
        for i in 0..ucoins.len() {
            let ucoin = ucoins[i].clone();
            wallet.ucoins.remove(i);
            wallet.scoins.push(ucoin);
        }

        let date = _tx.time.clone();
        let kind = YCoinKind::Transaction;
        let id = _tx.id;

        let outputs = _tx.outputs.clone();

        for idx in 0..outputs.len() {
            let output = outputs[idx].clone();
            let has_data = output.data.is_some();
            let tag = if !has_data {
                None
            } else {
                Some(output.data.clone().unwrap().tag)
            };
            let height = output.height;
            let amount = output.amount.clone();
            let coin = YCoin {
                date: date.clone(),
                sk: sks[idx],
                kind: kind,
                id: id,
                idx: idx as u32,
                height: height,
                has_data: has_data,
                tag: tag,
                amount: amount,
            };
            wallet.ucoins.push(coin);
            let _utxo = LibUTXO::from_output(&output, id, idx as u32);
            YUTXO::new(&_utxo).create(store)?;
        }

        let tx = YTransaction(_tx);
        tx.create(store)?;

        wallet.update(store, key)?;

        Ok(tx)
    }

    pub fn create_data<S: YStorage>(store: &mut S, key: YKey32, wallet_name: &str, to: YPublicKey, buf: &[u8], keep_data: bool) -> YHResult<YTransaction> {
        let data_sk = YSecretKey::random();
        let change_sk = YSecretKey::random();
        let change_pk = change_sk.to_public();
        
        let mut wallet = YWallet::get(store, key, wallet_name)?;
       
        let amount = YAmount::from_u64((buf.len()*2) as u64)?;

        let ucoins = if keep_data {
            wallet.select_coins_no_data(amount)?
        } else {
            wallet.select_coins(amount)?
        };
        
        let mut sks = Vec::new();
        let mut xs = Vec::new();
        for ucoin in ucoins.clone() {
            sks.push(ucoin.sk);
            xs.push(ucoin.sk.sk);
        }
        
        let mut utxos = Vec::new();

        for ucoin in ucoins.clone() {
            let id = ucoin.id;
            let idx = ucoin.idx;
            let height = ucoin.height;
            let recipient = ucoin.sk.to_public();
            let amount = ucoin.amount;
            let utxo = LibUTXO::new(id, idx, height, recipient, amount);
            utxos.push(utxo);
        }
        
        let _tx = LibTransaction::new_data(&data_sk, &change_sk,
                                           &to, &change_pk, buf,
                                           &utxos, &xs,
                                           None, None)?;

        // NB: change it if the coin selection
        //     is improved
        for i in 0..ucoins.len() {
            let ucoin = ucoins[i].clone();
            wallet.ucoins.remove(i);
            wallet.scoins.push(ucoin);
        }

        let date = _tx.time.clone();
        let kind = YCoinKind::Transaction;
        let id = _tx.id;

        let outputs = _tx.outputs.clone();

        for idx in 0..outputs.len() {
            let output = outputs[idx].clone();
            let has_data = output.data.is_some();
            let tag = if !has_data {
                None
            } else {
                Some(output.data.clone().unwrap().tag)
            };
            let height = output.height;
            let amount = output.amount.clone();
            let coin = YCoin {
                date: date.clone(),
                sk: sks[idx],
                kind: kind,
                id: id,
                idx: idx as u32,
                height: height,
                has_data: has_data,
                tag: tag,
                amount: amount,
            };
            wallet.ucoins.push(coin);
            if has_data {
                let _data = output.clone().data.unwrap();
                let data = YData::new(&_data)?;
                data.create(store)?;
            }
            let _utxo = LibUTXO::from_output(&output, id, idx as u32);
            YUTXO::new(&_utxo).create(store)?;
        }

        let tx = YTransaction(_tx);
        tx.create(store)?;

        wallet.update(store, key)?;

        Ok(tx)
    }

    pub fn delete<S: YStorage>(&self, store: &mut S) -> YHResult<()> {
        let store_buck = YBucket::Transactions.to_store_buck();
        let key = self.key()?;
        if !store.lookup(&store_buck, &key)? {
            return Err(YHErrorKind::NotFound.into());
        }
        store.delete(&store_buck, &key)
    }
}
