use crate::{model::{account::AssetAccountData, chain::QueryKey}};
use super::super::client::Client;

use wetee_runtime::{RuntimeCall, WeteeAssetsCall};
use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::MultiAddress;

/// 账户
pub struct WeteeAsset {
    pub base: Client,
}

impl WeteeAsset {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    pub async fn balance(
        &mut self,
        dao_id: u64,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let id = sr25519::Public::from_string(&address).unwrap().into();
        let balance:AssetAccountData<u128> = self.base.get_storage_double_map("Tokens", "Accounts", QueryKey::IntKey(dao_id), QueryKey::AccountId(id))
            .await
            .unwrap()
            .unwrap_or_default();

        Ok(balance)
    }

    pub async fn create_asset(
        &mut self,
        from: String,
        dao_id: u64,
        meta_name: String,
        meta_symbol: String,
        amount: u128,
        init_dao_asset: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::create_asset {
            dao_id,
            metadata: wetee_assets::DaoAssetMeta {
                name: meta_name.into(),
                symbol: meta_symbol.into(),
                decimals: 4,
            },
            amount,
            init_dao_asset,
        });
        self.base.send_and_sign(call,from).await
    }

    pub async fn set_existenial_deposit(
        &mut self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_existenial_deposit {
            dao_id,
            existenial_deposit: amount,
        });
        self.base.send_and_sign(call,from).await
    }

    pub async fn set_metadata(
        &mut self,
        from: String,
        dao_id: u64,
        metadata: wetee_assets::DaoAssetMeta,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_metadata { dao_id, metadata });
        self.base.send_and_sign(call,from).await
    }

    pub async fn burn(
        &mut self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::burn { dao_id, amount });
        self.base.send_and_sign(call,from).await
    }

    pub async fn transfer(
        &mut self,
        from: String,
        dao_id: u64,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::transfer {
            dao_id,
            amount,
            dest: MultiAddress::Id(dest.into()),
        });
        self.base.send_and_sign(call,from).await
    }

    pub async fn join_request(
        &mut self,
        from: String,
        dao_id: u64,
        share_expect: u32,
        existenial_deposit: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::join_request {
            dao_id,
            share_expect,
            existenial_deposit,
        });
        self.base.send_and_sign(call,from).await
    }
}
