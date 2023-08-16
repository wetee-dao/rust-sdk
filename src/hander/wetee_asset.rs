use super::super::client::Client;
use crate::model::{account::AssetAccountData, chain::QueryKey};

use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::MultiAddress;
use wetee_runtime::{RuntimeCall, WeteeAssetsCall};

/// 账户
pub struct WeteeAsset {
    pub base: Client,
}

impl WeteeAsset {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    /// 查询余额
    pub async fn balance(
        & self,
        dao_id: u64,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let id = sr25519::Public::from_string(&address).unwrap().into();
        let balance: AssetAccountData<u128> = self
            .base
            .get_storage_double_map(
                "Tokens",
                "Accounts",
                QueryKey::AccountId(id),
                QueryKey::U64Key(dao_id),
            )
            .await
            .unwrap()
            .unwrap_or_default();

        Ok(balance)
    }

    /// 创建资产
    pub async fn create_asset(
        & self,
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
        self.base.send_and_sign(call, from).await
    }

    /// 设置资产
    pub async fn set_existenial_deposit(
        & self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_existenial_deposit {
            dao_id,
            existenial_deposit: amount,
        });
        self.base.send_and_sign(call, from).await
    }

    /// 设置资产元数据
    pub async fn set_metadata(
        & self,
        from: String,
        dao_id: u64,
        metadata: wetee_assets::DaoAssetMeta,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_metadata { dao_id, metadata });
        self.base.send_and_sign(call, from).await
    }

    /// 销毁资产
    pub async fn burn(
        & self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::burn { dao_id, amount });
        self.base.send_and_sign(call, from).await
    }

    /// 转移资产
    pub async fn transfer(
        & self,
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
        self.base.send_and_sign(call, from).await
    }

    /// 加入 DAO
    pub async fn join_request(
        & self,
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
        self.base.send_and_sign(call, from).await
    }
}
