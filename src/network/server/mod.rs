use tokio_proto::TcpServer;
use std::net::SocketAddr;
use network::protocol::YMessageProto;
use network::service::YService;

pub enum YNetworkTransport {
    TCP=0,
}

pub struct YServer;

impl YServer {
    pub fn start(addr: SocketAddr, transport: YNetworkTransport) {
        match transport {
            YNetworkTransport::TCP => {
                let server = TcpServer::new(YMessageProto, addr);
                server.serve(|| Ok(YService))
            },
        }
    }
}
