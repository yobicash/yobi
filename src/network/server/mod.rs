use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use errors::*;

use models::peer::YPeer;
use config::YConfig;

pub trait YRPCRPCMethods {
    type API;

    fn info(&self) -> YHResult<String>; // TODO

    fn sync(&self) -> YHResult<()>;

    fn send_tx(&self, tx_hex: &str, fee_hex: &str) -> YHResult<()>;
    
    fn receive_tx(&self, tx_id: &str) -> YHResult<String>;
    
    fn send_cb(&self, cb_hex: &str) -> YHResult<()>;
    
    fn receive_cb(&self, cb_id: &str) -> YHResult<String>;
}

pub trait YRPCServer {
    type YRPCServerHandle;

    fn start(conf: &YConfig) -> YHResult<Self::YRPCServerHandle>;

    fn stop(handle: &mut Self::YRPCServerHandle) -> YHResult<()>;
}

pub struct YRPCServerHecate; // TODO

impl YRPCServer for YRPCServerHecate {
    type YRPCServerHandle = String; // TODO

    fn start(conf: &YConfig) -> YHResult<Self::YRPCServerHandle> {
        unreachable!()
    }

    fn stop(handle: &mut Self::YRPCServerHandle) -> YHResult<()> {
        unreachable!()
    }
}
