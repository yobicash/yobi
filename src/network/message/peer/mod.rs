use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::hash::sha::YSHA512;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use models::peer::YPeer;
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YListPeersReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub max: u32,
}

impl YListPeersReq {
    pub fn new(max: u32) -> YHResult<YListPeersReq> {
        let mut req = YListPeersReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListPeers,
            max: max,
        };
        req.id = req.calc_id()?;
        Ok(req)
    }

    pub fn check(&self) -> YHResult<()> {
        if self.id != self.calc_id()? {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidChecksum).into());
        }
        if self.version.major() > default_version().major() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidVersion(self.version.to_string())).into());
        }
        if self.time > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        if self.method != YRPCMethod::ListPeers {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.max as u32);
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.max as u32);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListPeersReq> {
        if buf.len() != 96 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let max = BigEndian::read_u32(b.get(92..96).unwrap());
        let ls_peers_req = YListPeersReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            max: max,
        };
        ls_peers_req.check()?;
        Ok(ls_peers_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YListPeersRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub count: u32,
    pub peers: Vec<YPeer>,
}

impl YListPeersRes {
    pub fn new(peers: &Vec<YPeer>) -> YHResult<YListPeersRes> {
        let mut res = YListPeersRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListPeers,
            count: peers.len() as u32,
            peers: peers.clone(),
        };
        res.id = res.calc_id()?;
        Ok(res)
    }

    pub fn check(&self) -> YHResult<()> {
        if self.id != self.calc_id()? {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidChecksum).into());
        }
        if self.version.major() > default_version().major() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidVersion(self.version.to_string())).into());
        }
        if self.time > YTime::now() {
            return Err(YHErrorKind::Lib(LibErrorKind::InvalidTime).into());
        }
        if self.method != YRPCMethod::ListPeers {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        if self.peers.len() != self.count as usize {
            return Err(YHErrorKind::InvalidLength.into());
        }
        for peer in self.peers.clone() {
            peer.check()?;
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let peer_buf = self.peers[i].to_bytes()?;
            let peer_size = peer_buf.len();
            buf.put_u32::<BigEndian>(peer_size as u32);
            buf.put(peer_buf);
        }
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
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

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListPeersRes> {
        if buf.len() < 96 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let count = BigEndian::read_u32(b.get(92..96).unwrap());
        let mut peers_buf = BytesMut::new();
        peers_buf.extend_from_slice(b.get(96..).unwrap());
        let mut peers = Vec::new();
        for i in 0..count as usize {
            let size = BigEndian::read_u32(peers_buf.get(i..i+4).unwrap()) as usize;
            peers.push(YPeer::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        let ls_peers_res = YListPeersRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            count: count,
            peers: peers,
        };
        ls_peers_res.check()?;
        Ok(ls_peers_res)
    }
}
