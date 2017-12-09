use libyobicash::errors::*;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;

use models::peer::YPeer;
use config::YConfig;

pub enum YCLICommand {
    Help {
        cmd_arg: Option<Box<YCLICommand>>,
    },
    Start {
        verbose_flag: Option<bool>,
        port_flag: Option<u16>,
        connect_flag: Option<YPeer>,
        config_flag: Option<String>,
        password_flag: Option<String>,
    },
    Connect {
        ip_arg: Option<YPeer>,
    },
    Disconnect,
    Sync {
        ip_arg: Option<YPeer>,
    },
    Stop,
}

pub trait YCLI {
    fn help(&self, config: &YConfig, cmd: Option<YCLICommand>) -> YResult<String>;

    fn start(&self, config: &YConfig, verb: bool, port: u16, peer: YPeer, conf: &str, pswd: &str) -> YResult<()>;

    fn connect(&self, config: &YConfig, peer: Option<YPeer>) -> YResult<()>;

    fn disconnect(&self, config: &YConfig) -> YResult<()>;

    fn sync(&self, config: &YConfig, peer: Option<YPeer>) -> YResult<()>;

    fn stop(&self, config: &YConfig) -> YResult<()>;
}

pub struct YCLIHecate;

impl YCLIHecate {
    pub fn start(config: Option<YConfig>) -> YResult<()> {
        unreachable!()
    }

    pub fn stop(&self) -> YResult<()> {
        unreachable!()
    }
}

impl YCLI for YCLIHecate {
    fn help(&self, config: &YConfig, cmd: Option<YCLICommand>) -> YResult<String> {
        unreachable!()
    }

    fn start(&self, config: &YConfig, verb: bool, port: u16, peer: YPeer, conf: &str, pswd: &str) -> YResult<()> { 
        unreachable!()
    }

    fn connect(&self, config: &YConfig, peer: Option<YPeer>) -> YResult<()> {
        unreachable!()
    }

    fn disconnect(&self, config: &YConfig) -> YResult<()> {
        unreachable!()
    }

    fn sync(&self, config: &YConfig, peer: Option<YPeer>) -> YResult<()> {
        unreachable!()
    }

    fn stop(&self, config: &YConfig) -> YResult<()> {
        unreachable!()
    }
}
