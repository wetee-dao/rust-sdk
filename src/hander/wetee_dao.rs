use crate::{model::{dao::Quarter, chain::QueryKey}};
use super::super::client::Client;

use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::AccountId32;
pub use wetee_dao::{DaoInfo, QuarterTask, Status};
use wetee_runtime::{
    AccountId, BlockNumber, RuntimeCall, WeteeAssetsCall, WeteeDaoCall,
};


/// DAO 模块
pub struct WeteeDAO {
    pub base: Client,
}

impl WeteeDAO {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    // 下一个 DAO ID
    pub async fn next_dao_id(&mut self) -> anyhow::Result<u64, anyhow::Error> {
        // 构建请求
        let result: u64 = self.base
            .get_storage_value("WeteeDAO", "NextDaoId").await
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
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeDAO(WeteeDaoCall::create_dao {
            name: name.into(),
            purpose: purpose.into(),
            meta_data: meta_data.into(),
        });
        self.base.send_and_sign(call,from).await
    }

    pub async fn member_list(&mut self, dao_id: u64) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        // 构建请求
        let result: Vec<AccountId> = self.base.get_storage_map("WeteeDAO", "Members", QueryKey::U64Key(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub async fn member_point(
        &mut self,
        dao_id: u64,
        member: String,
    ) -> anyhow::Result<u32, anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&member).unwrap().into();
        let result: u32 = self.base
            .get_storage_double_map("WeteeDAO", "MemberPoint", QueryKey::U64Key(dao_id), QueryKey::AccountId(who)).await
            .unwrap()
            .unwrap_or_else(|| 0);

        Ok(result)
    }

    pub async fn dao_info(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<DaoInfo<AccountId, BlockNumber>, anyhow::Error> {
        // 构建请求
        let result: DaoInfo<AccountId, BlockNumber> = self.base.get_storage_map("WeteeDAO", "Daos", QueryKey::U64Key(dao_id)).await
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
        self.base.send_and_sign(call,from).await
    }

    // DAO 里程碑
    pub async fn roadmap_list(
        &mut self,
        dao_id: u64,
        year: u32,
    ) -> anyhow::Result<Vec<Quarter>, anyhow::Error> {
        let mut results = vec![];
        for quarter in 1..5 {
            let tasks: Vec<QuarterTask<AccountId>> = self.base
                .get_storage_double_map("WeteeDAO", "RoadMaps", QueryKey::U64Key(dao_id), QueryKey::U32Key((year * 100 + quarter).into())).await
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

    //
    pub async fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        roadmap_id: u32,
        name: Vec<u8>,
        priority: u8,
        tags: Option<Vec<u8>>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeDAO(WeteeDaoCall::create_roadmap_task {
            dao_id,
            roadmap_id,
            name,
            priority,
            tags,
        });
        self.base.send_and_sign(call,from).await
    }

    // DAO 发行货币总量
    pub async fn total_issuance(&mut self, dao_id: u64) -> anyhow::Result<u128, anyhow::Error> {
        let result: u128 = self.base.get_storage_map("Tokens", "TotalIssuance", QueryKey::U64Key(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| 0);

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
