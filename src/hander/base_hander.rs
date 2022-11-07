use super::super::chain::*;
use super::super::client::Client;

#[derive(Debug)]
pub struct BaseHander {
  // 连接
  pub client: Client,
  // 无费模式
  pub feeless: bool,
  // 连接
  client_index: Option<u32>,
}

impl BaseHander {
  pub async fn get_chain_index(&mut self) -> usize {
    // 查到连接直接返回index
    if self.client_index.is_some() {
      return self.client_index.unwrap() as usize;
    }
    let i = get_api_index(self.client.uri.clone()).await;
    self.client_index = Some(i);
    i as usize
  }

  pub fn new(c: Client, feeless: bool) -> Self {
    Self {
      client: c,
      feeless: feeless,
      client_index: None,
    }
  }
}
