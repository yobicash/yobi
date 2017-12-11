use libyobicash::transaction::YTransaction;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use network::codec::transaction::*;
use std::io;

pub struct YTransactionProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for YTransactionProto {
    type Request = YTransaction;
    type Response = YTransaction;

    type Transport = Framed<T, YTransactionCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(YTransactionCodec))
    }
}
