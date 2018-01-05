use serde_json as json;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::message::ping::*;
use network::message::peer::*;
use network::message::data::*;
use network::message::transaction::*;
use network::message::coinbase::*;
use network::message::prefix::*;
use errors::*;

pub const YREQUEST_STATUS: u32 = 0;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YRequest {
    Ping(YPingReq),
    ListPeers(YListPeersReq),
    GetData(YGetDataReq),
    ListData(YListDataReq),
    GetTx(YGetTxReq),
    ConfirmTx(YConfirmTxReq),
    ListTxAncestors(YListTxAncestorsReq),
    GetCb(YGetCbReq),
}

impl YRequest {
    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        let mut req_buf = Vec::new();

        buf.put_u32::<BigEndian>(YMESSAGE_PREFIX);
        buf.put_u32::<BigEndian>(YREQUEST_STATUS);
        
        match *self {
           YRequest::Ping(ref req) => {
               buf.put_u32::<BigEndian>(0);
               req_buf = req.to_bytes()?;
           },
           YRequest::ListPeers(ref req) => {
               buf.put_u32::<BigEndian>(1);
               req_buf = req.to_bytes()?;
           },
           YRequest::GetData(ref req) => {
               buf.put_u32::<BigEndian>(2);
               req_buf = req.to_bytes()?;
           },
           YRequest::ListData(ref req) => {
               buf.put_u32::<BigEndian>(3);
               req_buf = req.to_bytes()?;
           },
           YRequest::GetTx(ref req) => {
               buf.put_u32::<BigEndian>(4);
               req_buf = req.to_bytes()?;
           },
           YRequest::ConfirmTx(ref req) => {
               buf.put_u32::<BigEndian>(5);
               req_buf = req.to_bytes()?;
           },
           YRequest::ListTxAncestors(ref req) => {
               buf.put_u32::<BigEndian>(6);
               req_buf = req.to_bytes()?;
           },
           YRequest::GetCb(ref req) => {
               buf.put_u32::<BigEndian>(7);
               req_buf = req.to_bytes()?;
           },
        }

        let req_size = req_buf.len() as u32;
        buf.put_u32::<BigEndian>(req_size);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YRequest> {
        let buf_len = buf.len();
        if buf_len < 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }

        let prefix = BigEndian::read_u32(&buf[0..4]);
        if prefix != YMESSAGE_PREFIX {
            return Err(YHErrorKind::InvalidMessagePrefix.into());
        }

        let status = BigEndian::read_u32(&buf[4..8]);
        if status != YREQUEST_STATUS {
            return Err(YHErrorKind::InvalidMessageStatus.into());
        }
        
        let kind = BigEndian::read_u32(&buf[8..12]);
        if kind > 7 {
            return Err(YHErrorKind::InvalidMessageKind.into());
        }
        
        let size = BigEndian::read_u32(&buf[12..16]) as usize;
        if buf_len != size + 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }

        let payload = &buf[16..];
        
        match kind {
            0 => {
                let req = YPingReq::from_bytes(&payload)?;
                Ok(YRequest::Ping(req))
            },
            1 => {
                let req = YListPeersReq::from_bytes(&payload)?;
                Ok(YRequest::ListPeers(req))
            },
            2 => {
                let req = YGetDataReq::from_bytes(&payload)?;
                Ok(YRequest::GetData(req))
            },
            3 => {
                let req = YListDataReq::from_bytes(&payload)?;
                Ok(YRequest::ListData(req))
            },
            4 => {
                let req = YGetTxReq::from_bytes(&payload)?;
                Ok(YRequest::GetTx(req))
            },
            5 => {
                let req = YConfirmTxReq::from_bytes(&payload)?;
                Ok(YRequest::ConfirmTx(req))
            },
            6 => {
                let req = YListTxAncestorsReq::from_bytes(&payload)?;
                Ok(YRequest::ListTxAncestors(req))
            },
            7 => {
                let req = YGetCbReq::from_bytes(&payload)?;
                Ok(YRequest::GetCb(req))
            },
            _ => {
                Err(YHErrorKind::InvalidRequest.into())
            }
        }
    }
    
    pub fn to_json(&self) -> YHResult<Vec<u8>> {
        let buf = json::to_vec(self)?;
        Ok(buf)
    }

    pub fn from_json(buf: &[u8]) -> YHResult<YRequest> {
        let req = json::from_slice(buf)?;
        Ok(req)
    }
}
