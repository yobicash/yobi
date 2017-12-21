use tokio_proto::TcpServer;
use std::net::{SocketAddr, SocketAddrV4};
use std::error::Error;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use config::*;
//use store::*;
use api::*;
use network::protocol::YMessageProto;
use network::service::YService;

#[derive(Debug)]
pub enum YServerTransport {
    TCP=0,
    UDP=1,
}

#[derive(Debug)]
pub enum YServerCodec {
    Binary=0,
    JSON=1,
}

#[derive(Debug)]
pub enum YServerStorage {
    Memory=0,
    Persistent=1,
}

#[derive(Debug)]
pub struct YServer {
    pub storage: YServerStorage,
    pub codec: YServerCodec,
    pub transport: YServerTransport,
}

impl YServer {
    pub fn new(s: YServerStorage, c: YServerCodec, t: YServerTransport) -> YServer {
        match c {
            YServerCodec::Binary => {
                panic!("Not implemented")
            },
            _ => {},
        }
        match t {
            YServerTransport::UDP => {
                panic!("Not implemented")
            },
            _ => {},
        }

        YServer {
            storage: s,
            codec: c,
            transport: t,
        }
    }

    pub fn start(&self) {
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

        let local = YConfig::read(None).unwrap().local;
        let _addr = SocketAddrV4::new(local.address, local.port);
        let addr = SocketAddr::V4(_addr);
        
        let server = TcpServer::new(YMessageProto, addr);
        // TODO: use tokio_core to avoid using FnOnce
        //       and douplicating configs adn imposing defaults
        server.serve(|| {
            let config = YConfig::read(None).unwrap();
            let read_only = false;
            let api = YAPI::new(config, read_only)
                .map_err(|err| IOError::new(IOErrorKind::Other, err.description()))?;
            let service = YService { api: api };
            Ok(service)
        });
    }
}
