use libyobicash::data::YData;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use network::codec::data::*;
use std::io;

pub struct YDataProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for YDataProto {
    type Request = YData;
    type Response = YData;

    type Transport = Framed<T, YDataCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(YDataCodec))
    }
}
