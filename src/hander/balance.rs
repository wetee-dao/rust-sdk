use std::str::FromStr;

use super::super::chain::*;
use super::super::client::Client;
use super::base_hander::BaseHander;
use crate::chain::asyoume::{self};

/// 账户
pub struct Balance {
    pub base: BaseHander,
}

impl Balance {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub async fn amount(
        &mut self,
        address: String,
    ) -> Result<(u128, u128, u128, u128), Box<dyn std::error::Error>> {
        // 获取区块链接口
        let apis = API_POOL.lock().unwrap();
        let api = apis.get(self.base.get_client_index()).unwrap();

        let account_id = sp_runtime::AccountId32::from_str(&address)?;
        let addr = asyoume::storage().balances().account(account_id);

        let value = api.storage().fetch_or_default(&addr, None).await?;

        Ok((
            value.free,
            value.reserved,
            value.fee_frozen,
            value.misc_frozen,
        ))
    }
}
