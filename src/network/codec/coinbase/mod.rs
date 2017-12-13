use libyobicash::errors::*;
use bytes::BytesMut; // TODO: use bytes in libyobicash
use tokio_io::codec::{Encoder, Decoder};
use network::message::coinbase::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YGetCbReqCodec;

impl Decoder for YGetCbReqCodec {
    type Item = YGetCbReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetCbReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetCbReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetCbReqCodec {
    type Item = YGetCbReq;
    type Error = IOError;

    fn encode(&mut self, msg: YGetCbReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YGetCbResCodec;

impl Decoder for YGetCbResCodec {
    type Item = YGetCbRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetCbRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetCbRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetCbResCodec {
    type Item = YGetCbRes;
    type Error = IOError;

    fn encode(&mut self, msg: YGetCbRes, buf: &mut BytesMut) -> IOResult<()> {
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
