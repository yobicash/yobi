use libyobicash::coinbase::YCoinbase;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use network::codec::coinbase::*;
use std::io;

pub struct YCoinbaseProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for YCoinbaseProto {
    type Request = YCoinbase;
    type Response = YCoinbase;

    type Transport = Framed<T, YCoinbaseCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(YCoinbaseCodec))
    }
}
