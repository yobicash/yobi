use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use models::peer::YPeer;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YConfig {
    pub password: String,
    pub db_path: String,
    pub seed: Vec<YPeer>,
    pub port: u16,
    pub price: u64,
}

impl Default for YConfig {
    fn default() -> YConfig {
        let password = YSHA512::hash(YRandom::bytes(64).as_slice()).to_hex();
        let seed = vec![YPeer::default()];
        let port = 2112;
        let price = 0;
        YConfig {
            password: password,
            db_path: "TODO".to_string(), //TODO
            seed: seed,
            port: port,
            price: price,
        }
    }
}

impl YConfig {
    pub fn new(pswd: &str, db_path: &str, seed: &Vec<YPeer>, port: u16, price: u64) -> YHResult<YConfig> {
        // TODO: check psdw complexity
        // TODO: check db_path
        // TODO: check seed peers
        Ok(YConfig {
            password: String::from(pswd),
            db_path: String::from(db_path),
            seed: seed.clone(),
            port: port,
            price: price,
        })
    }

    pub fn save(&self, path: Option<String>) -> YHResult<()> {
        unreachable!()
    }

    pub fn read(path: Option<String>) -> YHResult<YConfig> {
        unreachable!()
    }
}
