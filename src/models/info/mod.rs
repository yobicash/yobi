use libyobicash::utils::time::YTime;
use libyobicash::amount::YAmount;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YInfo {
    pub store_size: u32,
    pub online: bool,
    pub peers_count: u32,
    pub balance: YAmount,
    pub wallets_count: u32,
    pub txs_count: u32,
    pub data_count: u32,
}
