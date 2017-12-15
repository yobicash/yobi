use libyobicash::errors::*;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use network::message::message::*;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YMessageCodec;

impl Decoder for YMessageCodec {
    type Item = YMessage;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YMessage>> {
        let vec = buf.to_vec();
        YMessage::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|pr| Some(pr))
    }
}

impl Encoder for YMessageCodec {
    type Item = YMessage;
    type Error = IOError;

    fn encode(&mut self, msg: YMessage, buf: &mut BytesMut) -> IOResult<()> {
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
