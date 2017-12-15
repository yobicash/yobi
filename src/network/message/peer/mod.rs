use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use models::peer::YPeer;
use network::method::YMethod;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YListPeersReq {
    pub method: YMethod,
    pub max: u32,
}

impl YListPeersReq {
    pub fn new(max: u32) -> YListPeersReq {
        YListPeersReq {
            method: YMethod::ListPeers,
            max: max,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.method != YMethod::ListPeers {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.max as u32);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListPeersReq> {
        if buf.len() != 8 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let max = BigEndian::read_u32(b.get(4..).unwrap());
        let ls_peers_req = YListPeersReq {
            method: method,
            max: max,
        };
        ls_peers_req.check()?;
        Ok(ls_peers_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YListPeersRes {
    pub method: YMethod,
    pub count: u32,
    pub peers: Vec<YPeer>,
}

impl YListPeersRes {
    pub fn new(peers: &Vec<YPeer>) -> YListPeersRes {
        YListPeersRes {
            method: YMethod::ListPeers,
            count: peers.len() as u32,
            peers: peers.clone(),
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.method != YMethod::ListPeers {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        if self.peers.len() != self.count as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let peer_buf = self.peers[i].to_bytes()?;
            let peer_size = peer_buf.len();
            buf.put_u32::<BigEndian>(peer_size as u32);
            buf.put(peer_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListPeersRes> {
        if buf.len() < 8 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let method = BigEndian::read_u32(b.get(0..4).unwrap()).into();
        let count = BigEndian::read_u32(b.get(4..8).unwrap());
        let mut peers_buf = BytesMut::new();
        peers_buf.extend_from_slice(b.get(8..).unwrap());
        let mut peers = Vec::new();
        for i in 0..count as usize {
            let size = BigEndian::read_u32(peers_buf.get(i..i+4).unwrap()) as usize;
            peers.push(YPeer::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        let ls_peers_res = YListPeersRes {
            method: method,
            count: count,
            peers: peers,
        };
        ls_peers_res.check()?;
        Ok(ls_peers_res)
    }
}
