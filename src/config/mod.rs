use libyobicash::errors::*;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;

use models::peer::YPeer;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YConfig {
    pub password: String,
    pub seed: Vec<YPeer>,
    pub port: u16,
}

impl Default for YConfig {
    fn default() -> YConfig {
        let password = YSHA512::hash(YRandom::bytes(64).as_slice()).to_hex();
        let seed = vec![YPeer::default()];
        let port = 2112;
        YConfig {
            password: password,
            seed: seed,
            port: port,
        }
    }
}

impl YConfig {
    pub fn new(pswd: &str, seed: &Vec<YPeer>, port: u16) -> YConfig {
        YConfig {
            password: String::from(pswd),
            seed: seed.clone(),
            port: port,
        }
    }

    pub fn save(&self, path: Option<String>) -> YResult<()> {
        unreachable!()
    }

    pub fn read(path: Option<String>) -> YResult<YConfig> {
        unreachable!()
    }
}
