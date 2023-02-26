use super::super::client::Client;

#[derive(Debug)]
pub struct BaseHander {
    // 连接
    pub client: Client,
    // 无费模式
    pub feeless: bool,
}

impl BaseHander {
    pub fn get_client_index(&self) -> usize {
        self.client.index as usize
    }

    pub fn new(c: Client, feeless: bool) -> Self {
        Self {
            client: c,
            feeless: feeless,
        }
    }
}
