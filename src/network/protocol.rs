use libyobicash::data::YData;
use libyobicash::transaction::YTransaction;
use libyobicash::coinbase::YCoinbase;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use models::peer::*;
use network::codec::*;
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

pub struct YPeerProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for YPeerProto {
    type Request = YPeer;
    type Response = YPeer;

    type Transport = Framed<T, YPeerCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(YPeerCodec))
    }
}
