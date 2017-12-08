use libyobicash::errors::*;
use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::amount::YAmount;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YCoin {
    pub date: YTime,
    pub sk: YSecretKey,
    pub tx_id: YDigest64,
    pub idx: u32,
    pub amount: YAmount,
    pub has_data: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YWallet {
    pub name: String,
    pub balance: YAmount,
    pub stxos: Vec<YCoin>,
    pub utxos: Vec<YCoin>,
}
