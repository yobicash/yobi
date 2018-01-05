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

    pub fn init() -> YHResult<()> {
        YConfigDir::create_home()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct YConfig {
    pub password: String,
    pub light_mode: bool,
    pub seed: Vec<YHost>,
    pub host: YHost,
    pub max_conns: u16,
    pub price: u64,
}

impl Default for YConfig {
    fn default() -> YConfig {
        YConfig {
            password: YConfig::gen_pswd(),
            light_mode: YConfig::default_light_mode(),
            seed: YConfig::default_seed(),
            host: YConfig::default_host(),
            max_conns: YConfig::default_max_conns(),
            price: YConfig::default_price(),
        }
    }
}

impl YConfig {
    pub fn new(pswd: &str, light_mode: bool, seed: &Vec<YHost>, host: YHost, max_conns: u16, price: u64) -> YHResult<YConfig> {
        if pswd.len() < 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        Ok(YConfig {
            password: String::from(pswd),
            light_mode: light_mode,
            seed: seed.clone(),
            host: host,
            max_conns: max_conns,
            price: price,
        })
    }

    pub fn gen_pswd() -> String {
        YSHA512::hash(YRandom::bytes(32).as_slice()).to_hex()
    }

    pub fn default_light_mode() -> bool {
        false
    }

    pub fn default_seed() -> Vec<YHost> {
        vec![YConfig::default_host()]
    }

    pub fn default_host() -> YHost {
        YHost::default()
    }

    pub fn default_max_conns() -> u16 {
        8
    }

    pub fn default_price() -> u64 {
        0
    }

    pub fn db_path() -> YHResult<String> {
        YConfigDir::subdir("store")
    }

    pub fn to_json(&self) -> YHResult<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub fn from_json(s: &str) -> YHResult<YConfig> {
        let config = serde_json::from_str(s)?;
        Ok(config)
    }

    pub fn path() -> YHResult<String> {
        let mut path = PathBuf::new();
        path.push(&YConfigDir::home()?);
        path.push("config.json");
        let path_str = path
            .to_str()
            .unwrap()
            .to_string();
        Ok(path_str)
    }

    pub fn read() -> YHResult<YConfig> {
        let file = OpenOptions::new()
            .read(true)
            .open(YConfig::path()?)?;
        let mut json = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut json)?;
        YConfig::from_json(&json)
    }

    pub fn write(&self) -> YHResult<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(YConfig::path()?)?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }

    pub fn create(&self) -> YHResult<()> {
        YConfigDir::create_home()?;
        self.write()
    }

    pub fn create_default() -> YHResult<()> {
        let config = YConfig::default();
        config.create()
    }
}
