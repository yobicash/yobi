use serde::{Serialize, Deserialize};
use serde_json as json;
use std::net::TcpListener;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::fmt::Debug;
use config::*;
use store::common::*;
use errors::*;

#[derive(Debug)]
pub struct YServer {
    pub config: YConfig,
    pub storage_kind: YStorageKind,
    pub storage_mode: YStorageMode,
    pub difficulty: Option<u32>,
}

impl Default for YServer {
    fn default() -> YServer {
        YServer {
            config: YConfig::default(),
            storage_kind: YStorageKind::default(),
            storage_mode: YStorageMode::default(),
            difficulty: None,
        }
    }
}

impl YServer {
    pub fn new(config: YConfig,
               storage_kind: YStorageKind,
               storage_mode: YStorageMode,
               difficulty: Option<u32>) -> YHResult<YServer> {

        if let Some(difficulty) = difficulty {
            if difficulty < 3 || difficulty > 63 {
                return Err(YHErrorKind::InvalidDifficulty.into());
            }
        }

        let server = YServer {
            config: config,
            storage_kind: storage_kind,
            storage_mode: storage_mode,
            difficulty: difficulty,
        };

        Ok(server)
    }

    pub fn handle<R: Read, T>(reader: R) -> YHResult<Vec<u8>> 
        where for <'de>T: Serialize + Deserialize<'de> + Debug
    {
        let req: T = json::from_reader(reader).unwrap();
        
        // NB: So far just an echo

        let res_buf = json::to_vec(&req)?;
        Ok(res_buf)
    }

    pub fn run(self) {

        let addr = self.config.host.internal();
    
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        let max_conns = self.config.max_conns;
        let conns = Arc::new(Mutex::new(0u16));

        loop {
            match listener.accept() {
                Ok((connection, address)) => {
                    if *conns.lock().unwrap() == max_conns {
                        let e: YHError = YHErrorKind::MaxConnectionsReached.into();
                        panic!(e)
                    }

                    let count = conns.clone();

                    thread::spawn(move || {
                        *count.lock().unwrap() += 1;

                        // TODO
                        /*
                        let reader_conn = connection.try_clone().unwrap();
                        let req: T = json::from_reader(reader_conn).unwrap();

                        println!("request from {:?}: req {:?}", address, req);
                        
                        // NB: So far just an echo

                        let res_buf = json::to_vec(&req).unwrap();

                        connection.write(&res_buf).unwrap();
                        connection.flush().unwrap();

                        println!("reply to {:?}: {:?}", address, res_buf);
                        */

                    });
                },
                Err(_) => {
                    let e: YHError = YHErrorKind::FailedConnection.into();
                    panic!(e)
                }
            }
        }
    }
}
