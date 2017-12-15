use libyobicash::utils::random::YRandom;
use libyobicash::utils::time::YTime;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPeer {
    pub ip: [u8; 4],
    pub port: u16,
    pub last_time: YTime,
}

impl Default for YPeer {
    fn default() -> YPeer {
        let ip = [0, 0, 0, 0];
        let port = 2112;
        YPeer::new(ip, port)
    }
}

impl YPeer {
    pub fn new(ip: [u8; 4], port: u16) -> YPeer {
        YPeer {
            ip: ip,
            port: port,
            last_time: YTime::now(),
        }
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        unreachable!()
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YPeer> {
        unreachable!()
    }

    pub fn to_hex(&self) -> YHResult<String> {
        unreachable!()
    }

    pub fn from_hex(s: &str) -> YHResult<YPeer> {
        unreachable!()
    }
}
