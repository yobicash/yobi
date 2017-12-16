use errors::*;
use version::*;
use store::*;
use models::*;
use config::*;

pub struct YAPI {
    pub config: YConfig,
}

impl YAPI {
    pub fn open_store() {
        unreachable!()
    }

    pub fn close_store() {
        unreachable!()
    }

    pub fn reset_store() {
        unreachable!()
    }

    pub fn delete_store() {
        unreachable!()
    }

    pub fn ping() {
        unreachable!()
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
