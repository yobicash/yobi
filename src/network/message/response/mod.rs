use serde_json as json;
use bytes::{BytesMut, BufMut, BigEndian, ByteOrder};
use network::message::ping::*;
use network::message::peer::*;
use network::message::data::*;
use network::message::transaction::*;
use network::message::coinbase::*;
use network::message::error::*;
use network::message::prefix::*;
use errors::*;

pub const YRESPONSE_STATUS: u32 = 0;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YResponse {
    Ping(YPingRes),
    ListPeers(YListPeersRes),
    GetData(YGetDataRes),
    ListData(YListDataRes),
    GetTx(YGetTxRes),
    ConfirmTx(YConfirmTxRes),
    ListTxAncestors(YListTxAncestorsRes),
    GetCb(YGetCbRes),
    Error(YErrorRes),
}

impl YResponse {
    pub fn to_bytes(&self) -> YHResult<Vec<u8>> {
        let mut buf = BytesMut::new();
        let mut res_buf = Vec::new();

        buf.put_u32::<BigEndian>(YMESSAGE_PREFIX);
        buf.put_u32::<BigEndian>(YRESPONSE_STATUS);
        
        match *self {
           YResponse::Ping(ref res) => {
               buf.put_u32::<BigEndian>(0);
               res_buf = res.to_bytes()?;
           },
           YResponse::ListPeers(ref res) => {
               buf.put_u32::<BigEndian>(1);
               res_buf = res.to_bytes()?;
           },
           YResponse::GetData(ref res) => {
               buf.put_u32::<BigEndian>(2);
               res_buf = res.to_bytes()?;
           },
           YResponse::ListData(ref res) => {
               buf.put_u32::<BigEndian>(3);
               res_buf = res.to_bytes()?;
           },
           YResponse::GetTx(ref res) => {
               buf.put_u32::<BigEndian>(4);
               res_buf = res.to_bytes()?;
           },
           YResponse::ConfirmTx(ref res) => {
               buf.put_u32::<BigEndian>(5);
               res_buf = res.to_bytes()?;
           },
           YResponse::ListTxAncestors(ref res) => {
               buf.put_u32::<BigEndian>(6);
               res_buf = res.to_bytes()?;
           },
           YResponse::GetCb(ref res) => {
               buf.put_u32::<BigEndian>(7);
               res_buf = res.to_bytes()?;
           },
           YResponse::Error(ref res) => {
               buf.put_u32::<BigEndian>(8);
               res_buf = res.to_bytes()?;
           }
        }

        let res_size = res_buf.len() as u32;
        buf.put_u32::<BigEndian>(res_size);
        Ok(buf.to_vec())
    }

    pub fn from_bytes(buf: &[u8]) -> YHResult<YResponse> {
        let buf_len = buf.len();
        if buf_len < 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }

        let prefix = BigEndian::read_u32(&buf[0..4]);
        if prefix != YMESSAGE_PREFIX {
            return Err(YHErrorKind::InvalidMessagePrefix.into());
        }

        let status = BigEndian::read_u32(&buf[4..8]);
        if status != YRESPONSE_STATUS {
            return Err(YHErrorKind::InvalidMessageStatus.into());
        }
        
        let kind = BigEndian::read_u32(&buf[8..12]);
        if kind > 8 {
            return Err(YHErrorKind::InvalidMessageKind.into());
        }
        
        let size = BigEndian::read_u32(&buf[12..16]) as usize;
        if buf_len != size + 16 {
            return Err(YHErrorKind::InvalidLength.into());
        }

        let payload = &buf[16..];
        
        match kind {
            0 => {
                let res = YPingRes::from_bytes(&payload)?;
                Ok(YResponse::Ping(res))
            },
            1 => {
                let res = YListPeersRes::from_bytes(&payload)?;
                Ok(YResponse::ListPeers(res))
            },
            2 => {
                let res = YGetDataRes::from_bytes(&payload)?;
                Ok(YResponse::GetData(res))
            },
            3 => {
                let res = YListDataRes::from_bytes(&payload)?;
                Ok(YResponse::ListData(res))
            },
            4 => {
                let res = YGetTxRes::from_bytes(&payload)?;
                Ok(YResponse::GetTx(res))
            },
            5 => {
                let res = YConfirmTxRes::from_bytes(&payload)?;
                Ok(YResponse::ConfirmTx(res))
            },
            6 => {
                let res = YListTxAncestorsRes::from_bytes(&payload)?;
                Ok(YResponse::ListTxAncestors(res))
            },
            7 => {
                let res = YGetCbRes::from_bytes(&payload)?;
                Ok(YResponse::GetCb(res))
            },
            8 => {
                let res = YErrorRes::from_bytes(&payload)?;
                Ok(YResponse::Error(res))
            },
            _ => {
                Err(YHErrorKind::InvalidResponse.into())
            }
        }
    }
    
    pub fn to_json(&self) -> YHResult<Vec<u8>> {
        let buf = json::to_vec(self)?;
        Ok(buf)
    }

    pub fn from_json(buf: &[u8]) -> YHResult<YResponse> {
        let res = json::from_slice(buf)?;
        Ok(res)
    }
}
