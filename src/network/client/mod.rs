use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::io::prelude::*;
use std::io::ErrorKind as IOErrorKind;
use config::*;
use errors::*;

#[derive(Debug)]
pub struct YClient {
    pub address: SocketAddr,
    pub connection: TcpStream,
}

impl YClient {
    pub fn new(address: Option<SocketAddr>) -> YHResult<YClient> {
      
        let address = if address.is_some() {
            address.unwrap()
        } else {
            YConfig::read()?.host.internal()
        };

        let connection = TcpStream::connect(address)?;

        let client = YClient {
            address: address,
            connection: connection,
        };

        Ok(client)
    }

    fn read_reply(&mut self) -> YHResult<Vec<u8>> {
        let mut msg = Vec::new();
        loop {
            match self.connection.read_to_end(&mut msg) {
                Ok(_) => break,
                Err(ref e) if e.kind() == IOErrorKind::WouldBlock => {
                    let t = Duration::from_millis(10);
                    thread::sleep(t);
                },
                Err(e) => {
                    return Err(YHErrorKind::IO(e).into());
                },
            } 
        }
        
        // TODO: decode the reply
        
        Ok(msg)
    }

    fn handle_reply(&mut self) -> YHResult<()> {
        let reply = self.read_reply()?;
        
        // TODO handle the reply
        
        println!("reply from {:?}: {:?}", self.address, reply);
        Ok(())
    }

    pub fn send_request(&mut self, msg: &[u8]) -> YHResult<()> {
        self.connection.write(msg)?;
        self.connection.flush()?;

        self.handle_reply()?;

        Ok(())
    }
}
