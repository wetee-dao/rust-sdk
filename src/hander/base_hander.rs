use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use sp_core::sr25519;
use substrate_api_client::{rpc::WsRpcClient, Api, ExtrinsicSigner, PlainTipExtrinsicParams};
use wetee_runtime::{Runtime, Signature};

use super::super::client::Client;

// 连接中心
pub static POOL: Lazy<
    Mutex<
        HashMap<
            u32,
            Api<
                ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
                WsRpcClient,
                PlainTipExtrinsicParams<Runtime>,
                Runtime,
            >,
        >,
    >,
> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

#[derive(Debug)]
pub struct BaseHander {
    // 连接
    pub client: Client,
    // 无费模式
    pub feeless: bool,
}

impl BaseHander {
    pub fn get_client(
        &mut self,
    ) -> anyhow::Result<
        Api<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            WsRpcClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >,
        anyhow::Error,
    > {
        let url = self.client.get_url().unwrap();

        // 获取区块链接口
        let client = WsRpcClient::new(&url).unwrap();
        let api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            WsRpcClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();

        return Ok(api);
    }

    pub fn new(c: Client, feeless: bool) -> Self {
        Self { client: c, feeless }
    }
}
