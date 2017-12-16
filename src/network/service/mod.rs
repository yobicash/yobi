use tokio_service::Service;
use futures::{future, Future};
//use libyobicash::errors::YErrorKind as LibErrorKind; 
//use libyobicash::utils::version::*;
//use libyobicash::utils::time::*;
//use libyobicash::utils::random::*;
//use libyobicash::amount;
//use libyobicash::transaction::*;
//use libyobicash::proof::storage::*;
//use libyobicash::proof::work::*;
//use libyobicash::coinbase::*;
//use errors::*;
//use store::persistent::*;
//use store::memory::*;
//use models::data::*;
//use models::transaction::*;
//use models::coinbase::*;
//use models::coin::*;
//use models::peer::*;
//use models::wallet::*;
use network::message::message::*;
use std::io::Error as IOError;
//use std::io::ErrorKind as IOErrorKind;

pub struct YService;

impl YService {
    pub fn route(&self, req: YMessage) -> Box<Future<Item=YMessage, Error=IOError>> {
        unreachable!()
    }
}

impl Service for YService {
    type Request = YMessage;
    type Response = YMessage;

    type Error = IOError;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.route(req)
    }
}
