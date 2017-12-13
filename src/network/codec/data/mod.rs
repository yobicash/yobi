use libyobicash::errors::*;
use bytes::BytesMut; // TODO: use bytes in libyobicash
use tokio_io::codec::{Encoder, Decoder};
use network::message::data::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YListDataReqCodec;

impl Decoder for YListDataReqCodec {
    type Item = YListDataReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListDataReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListDataReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListDataReqCodec {
    type Item = YListDataReq;
    type Error = IOError;

    fn encode(&mut self, msg: YListDataReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YListDataResCodec;

impl Decoder for YListDataResCodec {
    type Item = YListDataRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListDataRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListDataRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListDataResCodec {
    type Item = YListDataRes;
    type Error = IOError;

    fn encode(&mut self, msg: YListDataRes, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YGetDataReqCodec;

impl Decoder for YGetDataReqCodec {
    type Item = YGetDataReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetDataReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetDataReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetDataReqCodec {
    type Item = YGetDataReq;
    type Error = IOError;

    fn encode(&mut self, msg: YGetDataReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YGetDataResCodec;

impl Decoder for YGetDataResCodec {
    type Item = YGetDataRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetDataRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetDataRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetDataResCodec {
    type Item = YGetDataRes;
    type Error = IOError;

    fn encode(&mut self, msg: YGetDataRes, buf: &mut BytesMut) -> IOResult<()> {
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
