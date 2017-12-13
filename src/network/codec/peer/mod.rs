use libyobicash::errors::*;
use bytes::BytesMut; // TODO: use bytes in libyobicash
use tokio_io::codec::{Encoder, Decoder};
use network::message::peer::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YListPeersReqCodec;

impl Decoder for YListPeersReqCodec {
    type Item = YListPeersReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListPeersReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListPeersReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListPeersReqCodec {
    type Item = YListPeersReq;
    type Error = IOError;

    fn encode(&mut self, msg: YListPeersReq, buf: &mut BytesMut) -> IOResult<()> {
        match msg.to_bytes() {
            Ok(msg_buf) => {
                buf.extend(msg_buf);
                Ok(())
            },
            Err(err) => {
                Err(IOError::new(IOErrorKind::Other, err.description()))
            },
        }
    }
}

pub struct YListPeersResCodec;

impl Decoder for YListPeersResCodec {
    type Item = YListPeersRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListPeersRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListPeersRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListPeersResCodec {
    type Item = YListPeersRes;
    type Error = IOError;

    fn encode(&mut self, msg: YListPeersRes, buf: &mut BytesMut) -> IOResult<()> {
        match msg.to_bytes() {
            Ok(msg_buf) => {
                buf.extend(msg_buf);
                Ok(())
            },
            Err(err) => {
                Err(IOError::new(IOErrorKind::Other, err.description()))
            },
        }
    }
}
