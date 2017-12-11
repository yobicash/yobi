use libyobicash::errors::*;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;

use models::peer::YPeer;
use config::YConfig;

pub trait YRPCMethods {
    type API;

    fn info(&self) -> YResult<String>; // TODO

    fn sync(&self) -> YResult<()>;

    fn send_tx(&self, tx_hex: &str, fee_hex: &str) -> YResult<()>;
    
    fn receive_tx(&self, tx_id: &str) -> YResult<String>;
    
    fn send_cb(&self, cb_hex: &str) -> YResult<()>;
    
    fn receive_cb(&self, cb_id: &str) -> YResult<String>;
}

pub trait YRPCServer {
    type YRPCServerHandle;

    fn start(conf: &YConfig) -> YResult<Self::YRPCServerHandle>;

    fn stop(handle: &mut Self::YRPCServerHandle) -> YResult<()>;
}

pub struct YRPCServerHecate; // TODO

impl YRPCServer for YRPCServerHecate {
    type YRPCServerHandle = String; // TODO

    fn start(conf: &YConfig) -> YResult<Self::YRPCServerHandle> {
        unreachable!()
    }

    fn stop(handle: &mut Self::YRPCServerHandle) -> YResult<()> {
        unreachable!()
    }
}
