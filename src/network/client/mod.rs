//use tokio_core::reactor::Core;
//use tokio_proto::TcpClient;
//use std::net::{SocketAddr, SocketAddrV4};
//use config::*;
//use network::protocol::YMessageProto;
use network::server::{YServerTransport, YServerCodec};

#[derive(Debug)]
pub struct YClient {
    pub codec: YServerCodec,
    pub transport: YServerTransport,
}

impl YClient {
    pub fn new(c: YServerCodec, t: YServerTransport) -> YClient {
        match c {
            YServerCodec::Binary => {
                panic!("Not implemented")
            },
            _ => {}
        }

        match t {
            YServerTransport::UDP => {
                panic!("Not implemented")
            },
            _ => {}
        }
        YClient {
            codec: c,
            transport: t,
        }
    }

    /* TODO: use tokio_core tcp_stream or *pnet*
    pub fn connect(&self, addr: Option<SocketAddr>) {
        match self.transport {
            YServerTransport::UDP => {
                panic!("Not implemented")
            },
            _ => {},
        }

        match self.codec {
            YServerCodec::Binary => {
                panic!("Not implemented")
            },
            _ => {},
        }

        let client = TcpClient::new(YMessageProto);
        
        let address = if addr.is_none() {
            let local = YConfig::read(None).unwrap().local;
            let _addr = SocketAddrV4::new(local.address, local.port);
            SocketAddr::V4(_addr)
        } else {
            addr.unwrap() 
        };
       
        let core = Core::new().unwrap();
        let handle = core.handle();

        client.connect(&address, &handle)
    }
    */
}
