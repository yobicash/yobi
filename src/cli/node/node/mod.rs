use std::process::*;
use config::*;
use store::common::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum YNodeStatus {
    NotStarted=0,
    Running=1,
    Stopped=2,
}

impl Default for YNodeStatus {
    fn default() -> YNodeStatus {
        YNodeStatus::NotStarted
    }
}

#[derive(Debug)]
pub struct YNode {
    pub config: YConfig,
    pub storage_kind: YStorageKind,
    pub storage_mode: YStorageMode,
    pub status: YNodeStatus,
    pub server: Option<Child>,
}

impl YNode {
    pub fn new(config: YConfig, storage_kind: YStorageKind, storage_mode: YStorageMode) -> YNode {
        YNode {
            config: config,
            storage_kind: storage_kind,
            storage_mode: storage_mode,
            status: YNodeStatus::default(),
            server: None,
        }
    }

    pub fn start() {
        unreachable!()
    }

    pub fn status() {
        unreachable!()
    }

    pub fn stop() {
        unreachable!()
    }
}
