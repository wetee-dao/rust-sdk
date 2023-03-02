use subxt::OnlineClient;

use crate::chain::WeteeConfig;

use super::super::client::Client;

#[derive(Debug)]
pub struct BaseHander {
    // 连接
    pub client: Client,
    // 无费模式
    pub feeless: bool,
}

impl BaseHander {
    pub async fn get_client(&mut self) -> anyhow::Result<OnlineClient<WeteeConfig>, anyhow::Error> {
        self.client.get_api().await
    }

    pub fn new(c: Client, feeless: bool) -> Self {
        Self {
            client: c,
            feeless: feeless,
        }
    }
}
