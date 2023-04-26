use anyhow::Ok;
use substrate_api_client::GetHeader;

use crate::chain::*;

/// 区块链连接
#[derive(Debug, Clone)]
pub struct Client {
    // u32
    pub index: usize,
    // 链接
    // api: Option<String>,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = get_api(uri.clone())?;
        Ok(Client { index: i })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client {
            index: index as usize,
        })
    }

    pub fn get_block_number(&mut self) -> Result<(u64, String), anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.index).unwrap();

        let header_hash = api.get_finalized_head().unwrap().unwrap();
        let h = api.get_header(Some(header_hash)).unwrap().unwrap();

        Ok((h.number, header_hash.to_string()))
    }
}
