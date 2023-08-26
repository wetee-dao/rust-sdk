use crate::model::chain::QueryKey;
use crate::model::dao::WithGov;

use super::{super::client::Client, wetee_gov::run_sudo_or_gov};
use sp_core::crypto::Ss58Codec;
use sp_core::sr25519;
use sp_runtime::AccountId32;
use wetee_org::GuildInfo;
use wetee_runtime::{AccountId, BlockNumber, RuntimeCall, WeteeGuildCall};

/// 账户
pub struct WeteeGuild {
    pub base: Client,
}

impl WeteeGuild {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    /// 工会列表
    pub async fn guild_list(
        & self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<GuildInfo<AccountId, BlockNumber>>, anyhow::Error> {
        
        // 构建请求
        let result: Vec<GuildInfo<AccountId, BlockNumber>> = self.base.get_storage_map("WeteeOrg", "Guilds", QueryKey::U64Key(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    /// 工会信息
    pub async fn guild_info(
        & self,
        dao_id: u64,
        index: u32,
    ) -> anyhow::Result<GuildInfo<AccountId, BlockNumber>, anyhow::Error> {
        // 构建请求
        let result: Vec<GuildInfo<AccountId, BlockNumber>> = self.base.get_storage_map("WeteeOrg", "Guilds", QueryKey::U64Key(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result.get(index as usize).unwrap().clone())
    }

    /// 创建工会
    pub async fn create_guild(
        &self,
        from: String,
        dao_id: u64,
        name: String,
        desc: String,
        meta_data: String,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&from).unwrap().into();
        let call = RuntimeCall::WeteeGuild(WeteeGuildCall::create_guild {
            name: name.into(),
            desc: desc.into(),
            meta_data: meta_data.into(),
            dao_id,
            creator: who,
        });

        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }

        self.base.send_and_sign(call,from).await
    }

    // 成员列表
    pub async fn member_list(
        &self,
        dao_id: u64,
        guild_id: u64,
    ) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        // 构建请求 
        let result: Vec<AccountId> = self.base
            .get_storage_double_map("WeteeOrg", "GuildMembers", QueryKey::U64Key(dao_id), QueryKey::U64Key(guild_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    /// 加入工会
    pub async fn guild_join_request(
        & self,
        from: String,
        dao_id: u64,
        guild_id: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&from).unwrap().into();
        let call = RuntimeCall::WeteeGuild(WeteeGuildCall::guild_join_request {
            dao_id,
            guild_id,
            who,
        });
        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }

        self.base.send_and_sign(call,from).await
    }
}
