use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use network::message::message::*;
use network::codec::message::*;
use std::io;

pub struct YMessageProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for YMessageProto {
    type Request = YMessage;
    type Response = YMessage;

    type Transport = Framed<T, YMessageCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(YMessageCodec))
    }
}
