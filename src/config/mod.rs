use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use serde_json;
use std::env::home_dir;
use std::fs::{create_dir_all, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::AsRef;
use network::host::YHost;
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

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct YConfig {
    pub password: String,
    pub db_path: String,
    pub seed: Vec<YHost>,
    pub local: YHost,
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
    pub fn new(pswd: &str, db_path: &str, seed: &Vec<YHost>, local: YHost, price: u64) -> YHResult<YConfig> {
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

    pub fn default_seed() -> Vec<YHost> {
        vec![YConfig::default_local()]
    }

    pub fn default_local() -> YHost {
        YHost::default()
    }

    pub fn default_price() -> u64 {
        0
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YConfig> {
        let config = serde_json::from_str(s)?;
        Ok(config)
    }

    pub fn path(home: Option<String>) -> YHResult<String> {
        let mut path = PathBuf::new();
        path.push(&home.unwrap_or(YConfigDir::home()?));
        path.push("config.json");
        let path_str = path
            .to_str()
            .unwrap()
            .to_string();
        Ok(path_str)
    }

    pub fn read(home: Option<String>) -> YHResult<YConfig> {
        let file = OpenOptions::new()
            .read(true)
            .open(YConfig::path(home)?)?;
        let mut json = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut json)?;
        YConfig::from_json(&json)
    }

    pub fn write(&self, home: Option<String>) -> YHResult<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(YConfig::path(home)?)?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }
}
