use anyhow::Ok;
use sp_core::sr25519;
use substrate_api_client::{rpc::WsRpcClient, Api, GetHeader, PlainTipExtrinsicParams};
use wetee_runtime::Runtime;

use crate::{chain::*};

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
        Ok(Client { index: index as usize })
    }

    pub fn get_url(&mut self) -> anyhow::Result<String, anyhow::Error> {
        let apis = API_POOL.lock().unwrap();
        if (self.index as usize) >= apis.len() {
            return Err(anyhow::anyhow!("index out of range"));
        }
        let url = apis.get(self.index as usize).unwrap();

        Ok(url.clone())
    }

    pub fn get_block_number(&mut self) -> Result<(u64, String), anyhow::Error> {
        // 获取区块链接口
        // let api = self.get_api().await?;
        let apis = API_POOL.lock().unwrap();
        let url = apis.get(self.index as usize).unwrap();

        let client = WsRpcClient::new(url).unwrap();
        let api = Api::<sr25519::Pair, _, PlainTipExtrinsicParams<Runtime>, Runtime>::new(client)
            .unwrap();

        let header_hash = api.get_finalized_head().unwrap().unwrap();
        let h = api.get_header(Some(header_hash)).unwrap().unwrap();

        Ok((h.number, header_hash.to_string()))
    }

    // pub async fn subscribe_block(&self) -> Result<(), subxt::Error> {
    //     // 获取区块链接口
    //     let apis = API_POOL.lock().unwrap();
    //     let api = apis.get(self.index as usize).unwrap();
    //     let mut block_sub = api.rpc().subscribe_finalized_blocks().await?;

    //     while let Some(Ok(block)) = block_sub.next().await {
    //         println!(
    //             "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
    //             block.number,
    //             block.hash(),
    //             block.parent_hash,
    //             block.state_root,
    //             block.extrinsics_root
    //         );
    //     }

    //     Ok(())
    // }

    // pub async fn unsubscribe(&self) -> Result<(), subxt::Error> {
    //     // 获取区块链接口
    //     let apis = API_POOL.lock().unwrap();
    //     let api = apis.get(self.index as usize).unwrap();
    //     let mut block_sub = api.rpc().subscribe_finalized_blocks().await?;

    //     while let Some(Ok(block)) = block_sub.next().await {
    //         println!(
    //             "block number: {} hash:{} parent:{} state root:{} extrinsics root:{}",
    //             block.number,
    //             block.hash(),
    //             block.parent_hash,
    //             block.state_root,
    //             block.extrinsics_root
    //         );
    //     }

    //     Ok(())
    // }
}
