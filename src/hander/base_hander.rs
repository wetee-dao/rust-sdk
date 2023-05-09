use super::super::client::Client;

#[derive(Debug)]
pub struct BaseHander {
    // 连接
    pub client: Client,
    // 无费模式
    pub feeless: bool,
}

impl BaseHander {
    // pub fn get_client(
    //     &mut self,
    // ) -> anyhow::Result<
    //     Api<
    //         ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
    //         TungsteniteRpcClient,
    //         PlainTipExtrinsicParams<Runtime>,
    //         Runtime,
    //     >,
    //     anyhow::Error,
    // > {
    //     let url = self.client.get_url().unwrap();

    //     // 获取区块链接口
    //     let client = TungsteniteRpcClient::new(&url).unwrap();
    //     let api = Api::<
    //         ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
    //         TungsteniteRpcClient,
    //         PlainTipExtrinsicParams<Runtime>,
    //         Runtime,
    //     >::new(client)
    //     .unwrap();

    //     return Ok(api);
    // }

    pub fn new(c: Client, feeless: bool) -> Self {
        Self { client: c, feeless }
    }
}
