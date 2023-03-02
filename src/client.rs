use subxt::OnlineClient;

use crate::chain::*;

/// 区块链连接
#[derive(Debug, Clone)]
pub struct Client {
    // u32
    pub index: u32,

    // 链接
    api: Option<OnlineClient<WeteeConfig>>,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = get_api_index(uri.clone())?;

        Ok(Client {
            index: i,
            api: None,
        })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client { index, api: None })
    }

    pub async fn get_api(&mut self) -> anyhow::Result<OnlineClient<WeteeConfig>, anyhow::Error> {
        if self.api.is_some() {
            return Ok(self.api.clone().unwrap());
        }

        let apis = API_POOL.lock().unwrap();
        let url = apis.get(self.index as usize).unwrap();
        let api = OnlineClient::<WeteeConfig>::from_url(url.clone()).await?;
        if let Err(_e) = wetee_chain::validate_codegen(&api) {
            println!("Generated code is not up to date with node we're connected to");
        }
        self.api = Some(api.clone());

        Ok(api)
    }

    pub fn get_url(&mut self) -> anyhow::Result<String, anyhow::Error> {
        let apis = API_POOL.lock().unwrap();
        let url = apis.get(self.index as usize).unwrap();

        Ok(url.clone())
    }

    pub async fn get_block_number(&mut self) -> Result<(u32, String), anyhow::Error> {
        // 获取区块链接口
        let api = self.get_api().await?;

        let mut blocks = api.rpc().subscribe_finalized_block_headers().await?;
        while let Some(Ok(block)) = blocks.next().await {
            return Ok((block.number, block.hash().to_string()));
        }

        Err(subxt::Error::Other("无法获取区块".to_owned()).into())
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
