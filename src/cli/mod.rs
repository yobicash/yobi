use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use errors::*;

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
    sync {
        ip_arg: Option<YPeer>,
    },
    Stop,
}

pub trait YCLI {
    fn help(&self, config: &YConfig, cmd: Option<YCLICommand>) -> YHResult<String>;

    fn start(&self, config: &YConfig, verb: bool, port: u16, peer: YPeer, conf: &str, pswd: &str) -> YHResult<()>;

    fn connect(&self, config: &YConfig, peer: Option<YPeer>) -> YHResult<()>;

    fn disconnect(&self, config: &YConfig) -> YHResult<()>;

    fn sync(&self, config: &YConfig, peer: Option<YPeer>) -> YHResult<()>;

    fn stop(&self, config: &YConfig) -> YHResult<()>;
}

pub struct YCLIHecate;

impl YCLIHecate {
    pub fn start(config: Option<YConfig>) -> YHResult<()> {
        unreachable!()
    }

    pub fn stop(&self) -> YHResult<()> {
        unreachable!()
    }
}

impl YCLI for YCLIHecate {
    fn help(&self, config: &YConfig, cmd: Option<YCLICommand>) -> YHResult<String> {
        unreachable!()
    }

    fn start(&self, config: &YConfig, verb: bool, port: u16, peer: YPeer, conf: &str, pswd: &str) -> YHResult<()> { 
        unreachable!()
    }

    fn connect(&self, config: &YConfig, peer: Option<YPeer>) -> YHResult<()> {
        unreachable!()
    }

    fn disconnect(&self, config: &YConfig) -> YHResult<()> {
        unreachable!()
    }

    fn sync(&self, config: &YConfig, peer: Option<YPeer>) -> YHResult<()> {
        unreachable!()
    }

    fn stop(&self, config: &YConfig) -> YHResult<()> {
        unreachable!()
    }
}
