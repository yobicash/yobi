use libyobicash::utils::version::YVersion;
use libyobicash::utils::time::YTime;
use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::data::YData;
use libyobicash::errors::*;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::method::YMethod;
use version::default_version;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YListDataReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub tx_id: YDigest64,
}

impl YListDataReq {
    pub fn new(tx_id: YDigest64) -> YListDataReq {
        YListDataReq {
            version: default_version(),
            method: YMethod::ListData,
            id: YRandom::u32(),
            time: YTime::now(),
            tx_id: tx_id,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::ListData {
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
        buf.put(self.tx_id.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListDataReq> {
        if buf.len() != 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_data_req = YListDataReq::default();
        ls_data_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_data_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_data_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_data_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_data_req.tx_id = YDigest64::from_bytes(b.get(44..).unwrap())?;
        ls_data_req.check()?;
        Ok(ls_data_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YListDataRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub count: u32,
    pub data: Vec<YData>,
}

impl YListDataRes {
    pub fn new(id: u32, count: u32, data: &Vec<YData>) -> YResult<YListDataRes> {
        if data.len() != count as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YListDataRes {
            version: default_version(),
            method: YMethod::ListData,
            id: id,
            time: YTime::now(),
            count: count,
            data: data.clone(),
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::ListData {
            return Err(YErrorKind::Other("Invalid method".to_string()).into());
        }
        if self.data.len() != self.count as usize {
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
            let data_buf = self.data[i].to_bytes()?;
            let data_size = data_buf.len();
            buf.put_u32::<BigEndian>(data_size as u32);
            buf.put(data_buf);
        }
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YListDataRes> {
        if buf.len() < 48 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_data_res = YListDataRes::default();
        ls_data_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_data_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_data_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_data_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_data_res.count = BigEndian::read_u32(b.get(44..48).unwrap());
        let mut data_buf = BytesMut::new();
        data_buf.extend_from_slice(b.get(48..).unwrap());
        for i in 0..ls_data_res.count as usize {
            let size = BigEndian::read_u32(data_buf.get(i..i+4).unwrap()) as usize;
            ls_data_res.data.push(YData::from_bytes(b.get(i+4..i+4+size).unwrap())?);
        }
        Ok(ls_data_res)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YGetDataReq {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub checksum: YDigest64,
}

impl YGetDataReq {
    pub fn new(checksum: YDigest64) -> YGetDataReq {
        YGetDataReq {
            version: default_version(),
            method: YMethod::GetData,
            id: YRandom::u32(),
            time: YTime::now(),
            checksum: checksum,
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetData {
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
        buf.put(self.checksum.to_bytes());
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetDataReq> {
        if buf.len() != 108 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut ls_data_req = YGetDataReq::default();
        ls_data_req.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        ls_data_req.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        ls_data_req.id = BigEndian::read_u32(b.get(32..36).unwrap());
        ls_data_req.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        ls_data_req.checksum = YDigest64::from_bytes(b.get(44..).unwrap())?;
        ls_data_req.check()?;
        Ok(ls_data_req)
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct YGetDataRes {
    pub version: YVersion,
    pub method: YMethod,
    pub id: u32,
    pub time: YTime,
    pub data: YData,
}

impl YGetDataRes {
    pub fn new(id: u32, count: u32, data: &YData) -> YGetDataRes {
        YGetDataRes {
            version: default_version(),
            method: YMethod::GetData,
            id: id,
            time: YTime::now(),
            data: data.clone(),
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.version != default_version() {
            let verstring = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(verstring).into());
        }
        if self.method != YMethod::GetData {
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
        buf.put(self.data.to_bytes()?);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YGetDataRes> {
        if buf.len() < 44 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut b = BytesMut::new();
        b.extend_from_slice(buf);
        let mut get_data_res = YGetDataRes::default();
        get_data_res.version = YVersion::from_bytes(b.get(4..28).unwrap())?;
        get_data_res.method = BigEndian::read_u32(b.get(28..32).unwrap()).into();
        get_data_res.id = BigEndian::read_u32(b.get(32..36).unwrap());
        get_data_res.time = YTime::from_bytes(b.get(36..44).unwrap())?;
        get_data_res.data = YData::from_bytes(b.get(44..).unwrap())?;
        get_data_res.check()?;
        Ok(get_data_res)
    }
}
