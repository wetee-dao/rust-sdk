use sp_core::H256;

use crate::chain::*;

/// 区块链连接
#[derive(Debug, Clone)]
pub struct Client {
    // u32
    pub index: u32,
}

impl Client {
    pub async fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = get_api_index(uri.clone()).await?;

        Ok(Client { index: i })
    }

    pub async fn get_block_number(&self) -> Result<(u32, String), subxt::Error> {
        // 获取区块链接口
        let apis = API_POOL.lock().unwrap();
        let api = apis.get(self.index as usize).unwrap();

        let mut blocks = api.rpc().subscribe_finalized_blocks().await?;

        while let Some(Ok(block)) = blocks.next().await {
            return Ok((block.number, block.hash().to_string()));
        }

        Err(subxt::Error::Other("无法获取区块".to_owned()))
    }

    pub async fn get_genesis_hash(&self) -> Result<H256, subxt::Error> {
        // 获取区块链接口
        let apis = API_POOL.lock().unwrap();
        let api = apis.get(self.index as usize).unwrap();

        api.rpc().genesis_hash().await
    }
}
