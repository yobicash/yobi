use libyobicash::errors::*;
use libyobicash::utils::random::YRandom;
use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPeer {
    pub ip: [u8; 4],
    pub port: u16,
    pub last_time: YTime,
}

impl Default for YPeer {
    fn default() -> YPeer {
        YPeer {
            ip: [0, 0, 0 , 0],
            port: 2112,
            last_time: YTime::now(),
        }
    }
}
