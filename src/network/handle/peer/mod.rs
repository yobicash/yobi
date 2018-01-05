use std::marker::PhantomData;
use api::*;
use store::*;
use network::rpc_method::*;
use network::message::ping::*;
use network::message::peer::*;
use network::message::data::*;
use network::message::transaction::*;
use network::message::coinbase::*;
use network::message::error::*;
use network::message::request::*;
use network::message::response::*;
use errors::*;

pub struct YPingHandle<M, P: YStorage> {
    _memory: PhantomData<M>,
    _persistent: PhantomData<P>,
}

impl<M, P: YStorage> YPingHandle<M, P> {
    pub fn handle(req: YRequest, api: &YAPI<M, P>) -> YHResult<YResponse> {
        match req {
            YRequest::Ping(req) => {
                unreachable!()
            },
            _ => {
                let err: YHError = YHErrorKind::InvalidRequest.into();
                let method = YRPCMethod::Ping;
                let res = YErrorRes::from_error(method, err)?;
                Ok(YResponse::Error(res)) 
            }
        }
    }

    pub fn handle_bytes(buf: &[u8], api: &YAPI<M, P>) -> YHResult<Vec<u8>> {
        let req = YRequest::from_bytes(buf)?;
        let res = YPingHandle::handle(req, api)?;
        res.to_bytes()
    }
    
    pub fn handle_json(obj: &[u8], api: &YAPI<M, P>) -> YHResult<Vec<u8>> {
        let req = YRequest::from_json(obj)?;
        let res = YPingHandle::handle(req, api)?;
        res.to_json()
    }
}
