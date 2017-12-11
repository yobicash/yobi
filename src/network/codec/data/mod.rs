use libyobicash::errors::*;
use libyobicash::data::YData;
use bytes::BytesMut; // TODO: use bytes in libyobicash
use tokio_io::codec::{Encoder, Decoder};
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::io::Result as IOResult;

pub struct YDataCodec;

impl Decoder for YDataCodec {
    type Item = YData;
    type Error = IOError;

    fn decode(&mut self, buf: &mut BytesMut) -> IOResult<Option<YData>> {
        let vec = buf.to_vec();
        // TODO: better error management
        YData::from_bytes(vec.as_slice())
            .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))
            .map(|tx| Some(tx))
    }
}

impl Encoder for YDataCodec {
    type Item = YData;
    type Error = IOError;

    fn encode(&mut self, msg: YData, buf: &mut BytesMut) -> IOResult<()> {
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
