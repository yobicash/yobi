use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use network::message::message::*;
use std::error::Error;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YMessageCodec;

impl Decoder for YMessageCodec {
    type Item = YMessage;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YMessage>> {
        let msg_str = String::from_utf8(buf.to_vec())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))?;
        YMessage::from_json(&msg_str)
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|msg| Some(msg))
    }
}

impl Encoder for YMessageCodec {
    type Item = YMessage;
    type Error = IOError;

    fn encode(&mut self, msg: YMessage, buf: &mut BytesMut) -> IOResult<()> {
        match msg.to_json() {
            Ok(msg_str) => {
                buf.extend(msg_str.as_bytes());
                Ok(())
            },
            Err(err) => {
                Err(IOError::new(IOErrorKind::Other, err.description()))
            },
        }
    }
}
