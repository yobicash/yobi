use tokio_service::Service;
//use futures::{future, Future};
use futures::Future;
use store::*;
//use models::*;
//use config::*;
use api::*;
use network::message::message::*;
use network::rpc_method::*;
//use errors::*;
use std::io::Error as IOError;

pub struct YService<M, P: YStorage> {
    pub api: YAPI<M, P>,
}

impl<M, P: YStorage> YService<M, P> {
    fn ping_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn ping_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_peers_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_peers_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_data_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_data_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_data_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_data_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_tx_ancestors_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn list_tx_ancestors_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_tx_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_tx_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn confirm_tx_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn confirm_tx_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_cb_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn get_cb_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn confirm_cb_req(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn confirm_cb_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    fn unknown_res(&self) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }

    pub fn route(&self, req: YMessage) -> Box<Future<Item=YMessage, Error=IOError>> {
        let method = req.method;
        let kind = req.kind;
        match method {
            YRPCMethod::Ping => {
                match kind {
                    YMessageKind::Request => {
                        self.ping_req()
                    },
                    YMessageKind::Response => {
                        self.ping_res()
                    },
                } 
            },
            YRPCMethod::ListPeers => {
                match kind {
                    YMessageKind::Request => {
                        self.list_peers_req()
                    },
                    YMessageKind::Response => {
                        self.list_peers_res()
                    },
                } 
            },
            YRPCMethod::ListData => {
                match kind {
                    YMessageKind::Request => {
                        self.list_data_req()
                    },
                    YMessageKind::Response => {
                        self.list_data_res()
                    },
                } 
            },
            YRPCMethod::GetData => {
                match kind {
                    YMessageKind::Request => {
                        self.get_data_req()
                    },
                    YMessageKind::Response => {
                        self.get_data_res()
                    },
                } 
            },
            YRPCMethod::ListTxAncestors => {
                match kind {
                    YMessageKind::Request => {
                        self.list_tx_ancestors_req()
                    },
                    YMessageKind::Response => {
                        self.list_tx_ancestors_res()
                    },
                } 
            },
            YRPCMethod::GetTx => {
                match kind {
                    YMessageKind::Request => {
                        self.get_tx_req()
                    },
                    YMessageKind::Response => {
                        self.get_tx_res()
                    },
                } 
            },
            YRPCMethod::ConfirmTx => {
                match kind {
                    YMessageKind::Request => {
                        self.confirm_tx_req()
                    },
                    YMessageKind::Response => {
                        self.confirm_tx_res()
                    },
                } 
            },
            YRPCMethod::GetCb => {
                match kind {
                    YMessageKind::Request => {
                        self.get_cb_req()
                    },
                    YMessageKind::Response => {
                        self.get_cb_res()
                    },
                } 
            },
            YRPCMethod::ConfirmCb => {
                match kind {
                    YMessageKind::Request => {
                        self.confirm_cb_req()
                    },
                    YMessageKind::Response => {
                        self.confirm_cb_res()
                    },
                } 
            },
            YRPCMethod::Unknown => {
                self.unknown_res()
            }
        }
    }
}

impl<M, P: YStorage> Service for YService<M, P> {
    type Request = YMessage;
    type Response = YMessage;

    type Error = IOError;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.route(req)
    }
}
