use libyobicash::utils::random::YRandom;
use libyobicash::utils::time::YTime;
use libyobicash::utils::version::YVersion;
use libyobicash::amount::YAmount;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::data::YData;
use libyobicash::input::YInput;
use libyobicash::output::YOutput;
use libyobicash::utxo::YUTXO;
use libyobicash::transaction::YTransaction;
use models::info::*;
use models::peer::*;
use models::wallet::*;
use models::output::*;
use errors::*;

// NB: for errors: std::io::Error
//     for YHResult would wrap std::io::Error in these cases
// NB: all the list RPCMethods return only the primary keys
pub struct YAPI;

impl YAPI {
    pub fn info() -> YHResult<YInfo> {
        Ok(YInfo {
            store_size: 100_000,
            online: true,
            peers_count: 10,
            balance: YAmount::from_u64(1_000)?,
            wallets_count: 10,
            txs_count: 2,
            data_count: 2,
        })
    }

    pub fn create_peer(ip: [u8; 4], port: u16) -> YHResult<YPeer> {
        let peer = YPeer {
            ip: ip,
            port: port,
            last_time: YTime::now(),
        };
        Ok(peer)
    }

    pub fn list_peers() -> YHResult<Vec<[u8; 4]>> {
        Ok(vec![[1, 2, 3, 4]])
    }

    pub fn create_wallet(name: String) -> YHResult<YWallet> {
        let wallet = YWallet {
            name: name,
            balance: YAmount::from_u64(1_000)?,
            scoins: vec![],
            ucoins: vec![],
        };
        Ok(wallet)
    }

    pub fn list_wallets() -> YHResult<Vec<String>> {
        Ok(vec!["mocked_wallet".to_string()])
    }

    pub fn get_wallet(name: String) -> YHResult<YWallet> {
        let stxo = YCoin {
            date: YTime::now(),
            sk: YSecretKey::random(),
            tx_id: YDigest64::from_bytes(YRandom::bytes(64).as_slice())?,
            idx: 0,
            amount: YAmount::from_u64(1_000)?,
        };
        let utxo = YCoin {
            date: YTime::now(),
            sk: YSecretKey::random(),
            tx_id: YDigest64::from_bytes(YRandom::bytes(64).as_slice())?,
            idx: 0,
            amount: YAmount::from_u64(1_000)?,
        };
        let wallet = YWallet {
            name: name,
            balance: YAmount::from_u64(1_000)?,
            scoins: vec![stxo],
            ucoins: vec![utxo],
        };
        Ok(wallet)
    }

    pub fn create_tx(wallet: String,
                     utxos: Vec<YUTXO>,
                     amount_outs: Vec<YAmountOutput>,
                     data_outs: Vec<YDataOutput>, 
                     activation: YTime) -> YHResult<YTransaction> {
        let tx = YTransaction {
            id: YDigest64::from_bytes(YRandom::bytes(64).as_slice())?,
            version: YVersion::default(),
            time: YTime::now(),
            height: YRandom::u64_range(1, 100),
            activation: Some(activation),
            inputs: vec![YInput::default()],
            // data is not stored directly in the db.
            // Use get_data with the checksum in the
            // outputs
            outputs: vec![YOutput::default()],
        };
        Ok(tx)
    }

    pub fn list_txs(wallet: String) -> YHResult<Vec<YDigest64>> {
        Ok(vec![YDigest64::from_bytes(YRandom::bytes(64).as_slice())?])
    }

    pub fn get_tx(wallet: String, id: YDigest64) -> YHResult<YTransaction> {
        let tx = YTransaction {
            id: id,
            version: YVersion::default(),
            time: YTime::now(),
            height: YRandom::u64_range(1, 100),
            activation: None,
            inputs: vec![YInput::default()],
            // data is not stored directly in the db.
            // Use get_data with the checksum in the
            // outputs
            outputs: vec![YOutput::default()],
        };
        Ok(tx)
    }

    pub fn list_data(wallet: String) -> YHResult<Vec<YDigest64>> {
        Ok(vec![YDigest64::from_bytes(YRandom::bytes(64).as_slice())?])
    }

    pub fn get_data(wallet: String, id: YDigest64) -> YHResult<YData> {
        Ok(YData::default()) 
    }
}
