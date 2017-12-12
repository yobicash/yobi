use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::elliptic::keys::YPublicKey;
use libyobicash::amount::YAmount;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use models::peer::YPeer;
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YListPeersReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub max: u32,
}

impl YListPeersReq {
    pub fn new(max: u32) -> YListPeersReq {
        YListPeersReq {
            version: default_version(),
            method: YMethod::ListPeers,
            id: YRandom::u32(),
            time: YTime::now(),
            max: max,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::ListPeers {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.max as u32);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListPeersReq> {
        if buf.len() != 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_peers_req = YListPeersReq::default();
        ls_peers_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_peers_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_peers_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_peers_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_peers_req.max = BigEndian::read_u32(b.get(44..).unwrap());
        ls_peers_req.check()?;
        Ok(ls_peers_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YListPeersRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub count: u32,
    pub peers: Vec<YPeer>,
}

impl YListPeersRes {
    pub fn new(id: u32, count: u32, peers: &Vec<YPeer>) -> YResult<YListPeersRes> {
        if peers.len() != count as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YListPeersRes {
            version: default_version(),
            method: YMethod::ListPeers,
            id: id,
            time: YTime::now(),
            count: count,
            peers: peers.clone(),
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
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
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.id as u32);
        buf.put(&self.time.to_bytes()[..]);
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
        if buf.len() < 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_peers_res = YListPeersRes::default();
        ls_peers_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_peers_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_peers_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_peers_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_peers_res.count = BigEndian::read_u32(b.get(44..48).unwrap());
        let mut peers_buf = BytesMut::new();
        peers_buf.extend_from_slice(b.get(48..).unwrap());
        for i in 0..ls_peers_res.count as usize {
            let size = BigEndian::read_u32(peers_buf.get(i..i+4).unwrap()) as usize;
            ls_peers_res.peers.push(YPeer::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        Ok(ls_peers_res)
    }
}
