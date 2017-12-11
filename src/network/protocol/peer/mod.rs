use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use models::peer::*;
use network::codec::peer::*;
use std::io;

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
