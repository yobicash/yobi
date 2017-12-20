use libyobicash::crypto::key::YKey32;
use libyobicash::amount::*;
use store::*;
use config::*;
use models::*;
use errors::*;

#[derive(Clone, Debug, Default)]
pub struct YInfo {
    pub config: YConfig,
    pub balance: YAmount,
    pub wallets_count: u32,
    pub ucoins_count: u32,
    pub scoins_count: u32,
    pub data_count: u32,
    pub transactions_count: u32,
    pub coinbases_count: u32,
}

impl YInfo {
    pub fn new() -> YInfo {
        YInfo::default()
    }

    pub fn get<S: YStorage>(store: &S, config: YConfig, key: YKey32) -> YHResult<YInfo> {
        let wallets_count = YWallet::count(store)?;
        let data_count = YData::count(store)?;
        let transactions_count = YTransaction::count(store)?;
        let coinbases_count = YCoinbase::count(store)?;
        let mut ucoins_count = 0;
        let mut scoins_count = 0;
        let mut balance = YAmount::zero();
        let wallets = YWallet::list(store, key, 0, wallets_count)?;
        for wallet in wallets {
            balance += wallet.balance;
            ucoins_count += wallet.ucoins.len() as u32;
            scoins_count += wallet.scoins.len() as u32;
        }
        let info = YInfo {
            config: config,
            balance: balance,
            wallets_count: wallets_count,
            scoins_count: scoins_count,
            ucoins_count: ucoins_count,
            data_count: data_count,
            transactions_count: transactions_count,
            coinbases_count: coinbases_count,
        };
        Ok(info)
    }
}
