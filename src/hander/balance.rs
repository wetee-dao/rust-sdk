use std::str::FromStr;

use subxt::ext::sp_runtime::AccountId32;

use super::super::client::Client;
use super::base_hander::BaseHander;
use crate::chain::wetee_chain;

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
        let api = self.base.get_client().await?;

        let account_id = AccountId32::from_str(&address)?;
        let addr = wetee_chain::storage().balances().account(account_id);

        let value = api.storage().fetch_or_default(&addr, None).await?;

        Ok((
            value.free,
            value.reserved,
            value.fee_frozen,
            value.misc_frozen,
        ))
    }
}
