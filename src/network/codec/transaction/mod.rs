use libyobicash::errors::*;
use bytes::BytesMut; // TODO: use bytes in libyobicash
use tokio_io::codec::{Encoder, Decoder};
use network::message::transaction::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YListTxAncestorsReqCodec;

impl Decoder for YListTxAncestorsReqCodec {
    type Item = YListTxAncestorsReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListTxAncestorsReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListTxAncestorsReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListTxAncestorsReqCodec {
    type Item = YListTxAncestorsReq;
    type Error = IOError;

    fn encode(&mut self, msg: YListTxAncestorsReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YListTxAncestorsResCodec;

impl Decoder for YListTxAncestorsResCodec {
    type Item = YListTxAncestorsRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YListTxAncestorsRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YListTxAncestorsRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YListTxAncestorsResCodec {
    type Item = YListTxAncestorsRes;
    type Error = IOError;

    fn encode(&mut self, msg: YListTxAncestorsRes, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YGetTxReqCodec;

impl Decoder for YGetTxReqCodec {
    type Item = YGetTxReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetTxReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetTxReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetTxReqCodec {
    type Item = YGetTxReq;
    type Error = IOError;

    fn encode(&mut self, msg: YGetTxReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YGetTxResCodec;

impl Decoder for YGetTxResCodec {
    type Item = YGetTxRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YGetTxRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YGetTxRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YGetTxResCodec {
    type Item = YGetTxRes;
    type Error = IOError;

    fn encode(&mut self, msg: YGetTxRes, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YConfirmTxReqCodec;

impl Decoder for YConfirmTxReqCodec {
    type Item = YConfirmTxReq;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YConfirmTxReq>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YConfirmTxReq::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YConfirmTxReqCodec {
    type Item = YConfirmTxReq;
    type Error = IOError;

    fn encode(&mut self, msg: YConfirmTxReq, buf: &mut BytesMut) -> IOResult<()> {
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

pub struct YConfirmTxResCodec;

impl Decoder for YConfirmTxResCodec {
    type Item = YConfirmTxRes;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YConfirmTxRes>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YConfirmTxRes::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YConfirmTxResCodec {
    type Item = YConfirmTxRes;
    type Error = IOError;

    fn encode(&mut self, msg: YConfirmTxRes, buf: &mut BytesMut) -> IOResult<()> {
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
