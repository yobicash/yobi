use libyobicash::errors::YErrorKind as LibErrorKind;
use libyobicash::utils::random::*;
use libyobicash::utils::time::*;
use libyobicash::utils::version::*;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::data::YData;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::rpc_method::YRPCMethod;
use version::*;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YListDataReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub tx_id: YDigest64,
}

impl YListDataReq {
    pub fn new(tx_id: YDigest64) -> YHResult<YListDataReq> {
        let mut req = YListDataReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListData,
            tx_id: tx_id,
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
        if self.method != YRPCMethod::ListData {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListDataReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let tx_id = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let ls_data_req = YListDataReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            tx_id: tx_id,
        };
        ls_data_req.check()?;
        Ok(ls_data_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YListDataRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub count: u32,
    pub data: Vec<YData>,
}

impl YListDataRes {
    pub fn new(data: &Vec<YData>) -> YHResult<YListDataRes> {
        let mut res = YListDataRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::ListData,
            count: data.len() as u32,
            data: data.clone(),
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
        if self.method != YRPCMethod::ListData {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        if self.data.len() != self.count as usize {
            return Err(YHErrorKind::InvalidLength.into());
        }
        for data in self.data.clone() {
            data.check()?;
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let data_buf = self.data[i].to_bytes()?;
            let data_size = data_buf.len();
            buf.put_u32::<BigEndian>(data_size as u32);
            buf.put(data_buf);
        }
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put_u32::<BigEndian>(self.count as u32);
        for i in 0..self.count as usize {
            let data_buf = self.data[i].to_bytes()?;
            let data_size = data_buf.len();
            buf.put_u32::<BigEndian>(data_size as u32);
            buf.put(data_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YListDataRes> {
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
        let mut data_buf = BytesMut::new();
        data_buf.extend_from_slice(b.get(96..).unwrap());
        let mut data = Vec::new();
        for i in 0..count as usize {
            let size = BigEndian::read_u32(data_buf.get(i..i+4).unwrap()) as usize;
            data.push(YData::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        let ls_data_res = YListDataRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            count: count,
            data: data,
        };
        Ok(ls_data_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YGetDataReq {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub checksum: YDigest64,
}

impl YGetDataReq {
    pub fn new(checksum: YDigest64) -> YHResult<YGetDataReq> {
        let mut req = YGetDataReq {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetData,
            checksum: checksum,
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
        if self.method != YRPCMethod::GetData {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.checksum.to_bytes());
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put_u32::<BigEndian>(self.nonce);
        buf.put(self.method.to_bytes());
        buf.put(self.checksum.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetDataReq> {
        if buf.len() != 156 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let checksum = YDigest64::from_bytes(b.get(92..156).unwrap())?;
        let get_data_req = YGetDataReq {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            checksum: checksum,
        };
        get_data_req.check()?;
        Ok(get_data_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct YGetDataRes {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub nonce: u32,
    pub method: YRPCMethod,
    pub data: YData,
}

impl YGetDataRes {
    pub fn new(data: &YData) -> YHResult<YGetDataRes> {
        let mut res = YGetDataRes {
            id: YDigest64::default(),
            version: default_version(),
            time: YTime::now(),
            nonce: YRandom::u32(),
            method: YRPCMethod::GetData,
            data: data.clone(),
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
        if self.method != YRPCMethod::GetData {
            return Err(YHErrorKind::InvalidRPCMethod.into());
        }
        self.data.check()?;
        Ok(())
    }

    pub fn calc_id(&self) -> YHResult<YDigest64> {
        let mut buf = BytesMut::new();
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.data.to_bytes()?);
        Ok(YSHA512::hash(&buf.to_vec()))
    }

    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        self.check()?;
        let mut buf = BytesMut::new();
        buf.put(self.id.to_bytes());
        buf.put(&self.version.to_bytes()?[..]);
        buf.put(&self.time.to_bytes()[..]);
        buf.put(self.method.to_bytes());
        buf.put(self.data.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YGetDataRes> {
        if buf.len() < 192 {
            return Err(YHErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let id = YDigest64::from_bytes(b.get(0..64).unwrap())?;
        let version = YVersion::from_bytes(b.get(64..76).unwrap())?;
        let time = YTime::from_bytes(b.get(76..84).unwrap())?;
        let nonce = BigEndian::read_u32(b.get(84..88).unwrap());
        let method = BigEndian::read_u32(b.get(88..92).unwrap()).into();
        let data = YData::from_bytes(b.get(92..).unwrap())?;
        let get_data_res = YGetDataRes {
            id: id,
            version: version,
            time: time,
            nonce: nonce,
            method: method,
            data: data,
        };
        get_data_res.check()?;
        Ok(get_data_res)
    }
}
