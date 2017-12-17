use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use std::env::home_dir;
use std::fs::create_dir_all;
use std::path::Path;
use std::convert::AsRef;
use network::address::YAddress;
use errors::*;

pub struct YConfigDir;

impl YConfigDir {
    pub fn home() -> YHResult<String> {
        let mut dir = home_dir().unwrap();
        dir.push(".");
        dir.push("yobicash");
        let dir_str = dir
            .to_str()
            .unwrap()
            .to_string();
        Ok(dir_str)
    }

    pub fn home_exists() -> bool {
        let mut dir = home_dir().unwrap();
        dir.push(".");
        dir.push("yobicash");
        dir.exists() 
    }

    pub fn create_home() -> YHResult<()> {
        if Self::home_exists() {
            return Ok(());
        }
        create_dir_all(Self::home()?)?;
        Ok(())
    }

    pub fn subdir<P: AsRef<Path>>(path: P) -> YHResult<String> {
        let mut dir = home_dir().unwrap();
        dir.push(".");
        dir.push("yobicash");
        dir.push(path);
        let dir_str = dir
            .to_str()
            .unwrap()
            .to_string();
        Ok(dir_str)
    }

    pub fn subdir_exists<P: AsRef<Path>>(path: P) -> bool {
        let mut dir = home_dir().unwrap();
        dir.push(".");
        dir.push("yobicash");
        dir.push(path);
        dir.exists()
    }

    pub fn create_subdir<P: AsRef<Path>>(path: P) -> YHResult<()> {
        if Self::subdir_exists(&path) {
            return Ok(());
        }
        create_dir_all(Self::subdir(&path)?)?;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YConfig {
    pub password: String,
    pub db_path: String,
    pub seed: Vec<YAddress>,
    pub local: YAddress,
    pub price: u64,
}

impl Default for YConfig {
    fn default() -> YConfig {
        YConfig {
            password: YConfig::gen_pswd(),
            db_path: YConfig::default_db_path().unwrap(),
            seed: YConfig::default_seed(),
            local: YConfig::default_local(),
            price: YConfig::default_price(),
        }
    }
}

impl YConfig {
    pub fn new(pswd: &str, db_path: &str, seed: &Vec<YAddress>, local: YAddress, price: u64) -> YHResult<YConfig> {
        if pswd.len() < 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        Ok(YConfig {
            password: String::from(pswd),
            db_path: YConfigDir::subdir(db_path)?,
            seed: seed.clone(),
            local: local,
            price: price,
        })
    }

    pub fn gen_pswd() -> String {
        YSHA512::hash(YRandom::bytes(32).as_slice()).to_hex()
    }

    pub fn default_db_path() -> YHResult<String> {
        YConfigDir::subdir("store")
    }

    pub fn default_seed() -> Vec<YAddress> {
        vec![YConfig::default_local()]
    }

    pub fn default_local() -> YAddress {
        YAddress::default()
    }

    pub fn default_price() -> u64 {
        0
    }

    pub fn save(&self, path: Option<String>) -> YHResult<()> {
        unreachable!()
    }

    pub fn read(path: Option<String>) -> YHResult<YConfig> {
        unreachable!()
    }
}
