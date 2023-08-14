use super::{super::client::Client, wetee_gov::run_sudo_or_gov};
use crate::model::{chain::QueryKey, dao::Quarter};

use crate::model::dao::WithGov;
use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::AccountId32;
pub use wetee_org::{App, OrgApp};
pub use wetee_org::{OrgInfo, QuarterTask, Status};
use wetee_runtime::{AccountId, BlockNumber, RuntimeCall, WeteeOrgCall, WeteeAssetsCall};

/// DAO 模块
pub struct WeteeOrg {
    pub base: Client,
}

impl WeteeOrg {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    // 下一个 DAO ID
    pub async fn next_dao_id(&mut self) -> anyhow::Result<u64, anyhow::Error> {
        // 构建请求
        let result: u64 = self
            .base
            .get_storage_value("WeteeOrg", "NextDaoId")
            .await
            .unwrap()
            .unwrap_or_else(|| 5000);

        Ok(result)
    }

    // 创建 DAO
    pub async fn create_dao(
        &mut self,
        from: String,
        name: String,
        purpose: String,
        meta_data: String,
        desc: String,
        im_api: String,
        bg: String,
        logo: String,
        img: String,
        home_url: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::create_dao {
            name: name.into(),
            purpose: purpose.into(),
            meta_data: meta_data.into(),
            desc: desc.into(),
            im_api: im_api.into(),
            bg: bg.into(),
            logo: logo.into(),
            img: img.into(),
            home_url: home_url.into(),
        });
        self.base.send_and_sign(call,from).await
    }

    // DAO 组织
    pub async fn orgs(
        &mut self,
    ) -> anyhow::Result<Vec<(String, OrgInfo<AccountId, u64>)>, anyhow::Error> {
        let results: Vec<(String, OrgInfo<AccountId, u64>)> = self
            .base
            .get_storage_map_all("WeteeOrg", "Daos")
            .await
            .unwrap();

        Ok(results)
    }

    // 成员列表
    pub async fn member_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        // 构建请求
        let result: Vec<AccountId> = self
            .base
            .get_storage_map("WeteeOrg", "Members", QueryKey::U64Key(dao_id))
            .await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 成员声誉
    pub async fn member_point(
        &mut self,
        dao_id: u64,
        member: String,
    ) -> anyhow::Result<u32, anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&member).unwrap().into();
        let result: u32 = self
            .base
            .get_storage_double_map(
                "WeteeOrg",
                "MemberPoint",
                QueryKey::U64Key(dao_id),
                QueryKey::AccountId(who),
            )
            .await
            .unwrap()
            .unwrap_or_else(|| 0);

        Ok(result)
    }

    // DAO 信息
    pub async fn dao_info(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<OrgInfo<AccountId, BlockNumber>, anyhow::Error> {
        // 构建请求
        let result: OrgInfo<AccountId, BlockNumber> = self
            .base
            .get_storage_map("WeteeOrg", "Daos", QueryKey::U64Key(dao_id))
            .await
            .unwrap()
            .unwrap();

        Ok(result)
    }

    // 加入 DAO
    pub async fn join(
        &mut self,
        from: String,
        dao_id: u64,
        share_expect: u32,
        value: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::join_request {
            dao_id,
            share_expect,
            existenial_deposit: value.into(),
        });
        self.base.send_and_sign(call, from).await
    }

    // DAO 里程碑
    pub async fn roadmap_list(
        &mut self,
        dao_id: u64,
        year: u32,
    ) -> anyhow::Result<Vec<Quarter>, anyhow::Error> {
        let mut results = vec![];
        for quarter in 1..5 {
            let tasks: Vec<QuarterTask<AccountId>> = self
                .base
                .get_storage_double_map(
                    "WeteeOrg",
                    "RoadMaps",
                    QueryKey::U64Key(dao_id),
                    QueryKey::U32Key((year * 100 + quarter).into()),
                )
                .await
                .unwrap()
                .unwrap_or_else(|| vec![]);

            results.push(Quarter {
                year,
                quarter,
                tasks,
            });
        }

        Ok(results)
    }

    // 创建任务
    pub async fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        roadmap_id: u32,
        name: Vec<u8>,
        priority: u8,
        tags: Option<Vec<u8>>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::create_roadmap_task {
            dao_id,
            roadmap_id,
            name,
            priority,
            tags,
        });
        self.base.send_and_sign(call, from).await
    }

    // DAO 发行货币总量
    pub async fn total_issuance(&mut self, dao_id: u64) -> anyhow::Result<u128, anyhow::Error> {
        let result: u128 = self
            .base
            .get_storage_map("Tokens", "TotalIssuance", QueryKey::U64Key(dao_id))
            .await
            .unwrap()
            .unwrap_or_else(|| 0);

        Ok(result)
    }

    // 创建应用
    pub async fn create_app(
        &mut self,
        from: String,
        name: String,
        desc: String,
        icon: String,
        url: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::create_app {
            name: name.into(),
            desc: desc.into(),
            icon: icon.into(),
            url: url.into(),
        });
        self.base.send_and_sign(call, from).await
    }

    // 应用状态
    pub async fn update_app_status(
        &mut self,
        from: String,
        app_id: u64,
        status: u8,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::update_app_status {
            app_id,
            status: if status == 0 {
                Status::Active
            } else {
                Status::InActive
            },
        });
        self.base.send_and_sign(call, from).await
    }

    // 应用集成
    pub async fn org_integrate_app(
        &mut self,
        from: String,
        dao_id: u64,
        app_id: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::org_integrate_app { dao_id, app_id });
        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }
        self.base.send_and_sign(call, from).await
    }

    // 更新应用状态
    pub async fn update_org_app_status(
        &mut self,
        from: String,
        dao_id: u64,
        app_id: u64,
        status: u8,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeOrg(WeteeOrgCall::update_org_app_status {
            dao_id,
            app_id,
            status: if status == 0 {
                Status::Active
            } else {
                Status::InActive
            },
        });
        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }
        self.base.send_and_sign(call, from).await
    }

    // DAO 应用
    pub async fn app_hubs(
        &mut self,
    ) -> anyhow::Result<Vec<(String, App<AccountId>)>, anyhow::Error> {
        let results: Vec<(String, App<AccountId>)> = self
            .base
            .get_storage_map_all("WeteeOrg", "AppHubs")
            .await
            .unwrap();

        Ok(results)
    }

    // 组织应用
    pub async fn org_apps(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<OrgApp<BlockNumber>>, anyhow::Error> {
        let result: Vec<OrgApp<BlockNumber>> = self
            .base
            .get_storage_map("WeteeOrg", "OrgApps", QueryKey::U64Key(dao_id))
            .await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }
}

// // 等待区块确认
// loop {
//     let mut goto = true;
//     let tx_wrap = sub.next().unwrap();
//     if let Err(e) = tx_wrap {
//         println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
//         let string_error = format!("{:?}", e);
//         return Err(anyhow::anyhow!(string_error));
//     }
//     let tx = tx_wrap.unwrap();
//     match tx {
//         TransactionStatus::Future => {}
//         TransactionStatus::Ready => todo!(),
//         TransactionStatus::Broadcast(_) => todo!(),
//         TransactionStatus::InBlock(_) => todo!(),
//         TransactionStatus::Retracted(_) => todo!(),
//         TransactionStatus::FinalityTimeout(_) => todo!(),
//         TransactionStatus::Finalized(_) => {
//             println!("[+] Extrinsic got included in block");
//             goto = true;
//             break;
//         }
//         TransactionStatus::Usurped(_) => todo!(),
//         TransactionStatus::Dropped => todo!(),
//         TransactionStatus::Invalid => todo!(),
//     }
//     if !goto {
//         break;
//     }
// }
