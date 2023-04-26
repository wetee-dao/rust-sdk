use crate::{account, model::dao::Quarter, chain::API_POOL_NEW};

use super::super::client::Client;
use super::base_hander::BaseHander;
use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::AccountId32;
pub use wetee_dao::{DaoInfo, QuarterTask, Status};
use wetee_runtime::{
    AccountId, BlockNumber, Runtime, RuntimeCall, Signature, WeteeAssetsCall, WeteeDaoCall,
};

use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// DAO 模块
pub struct WeteeDAO {
    pub base: BaseHander,
}

impl WeteeDAO {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    // 下一个 DAO ID
    pub fn next_dao_id(&mut self) -> anyhow::Result<u64, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();

        // 构建请求
        let result: u64 = api
            .get_storage_value("WeteeDAO", "NextDaoId", None)
            .unwrap()
            .unwrap_or_else(|| 5000);

        Ok(result)
    }

    // 创建 DAO
    pub fn create_dao(
        &mut self,
        from: String,
        name: String,
        purpose: String,
        meta_data: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeDAO(WeteeDaoCall::create_dao {
            name: name.into(),
            purpose: purpose.into(),
            meta_data: meta_data.into(),
        });
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    pub fn member_list(&mut self, dao_id: u64) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();


        // 构建请求
        let result: Vec<AccountId> = api
            .get_storage_map("WeteeDAO", "Members", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub fn member_point(
        &mut self,
        dao_id: u64,
        member: String,
    ) -> anyhow::Result<u32, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();


        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&member).unwrap().into();
        let result: u32 = api
            .get_storage_double_map("WeteeDAO", "MemberPoint", dao_id, who, None)
            .unwrap()
            .unwrap_or_else(|| 0);

        Ok(result)
    }

    pub fn dao_info(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<DaoInfo<AccountId, BlockNumber>, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();


        // 构建请求
        let result: DaoInfo<AccountId, BlockNumber> = api
            .get_storage_map("WeteeDAO", "Daos", dao_id, None)
            .unwrap()
            .unwrap();

        Ok(result)
    }

    // 加入 DAO
    pub fn join(
        &mut self,
        from: String,
        dao_id: u64,
        share_expect: u32,
        value: u64,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::join_request {
            dao_id,
            share_expect,
            existenial_deposit: value.into(),
        });
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    // DAO 里程碑
    pub fn roadmap_list(
        &mut self,
        dao_id: u64,
        year: u32,
    ) -> anyhow::Result<Vec<Quarter>, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();

        let mut results = vec![];
        for quarter in 1..5 {
            let tasks: Vec<QuarterTask<AccountId>> = api
                .get_storage_double_map("WeteeDAO", "RoadMaps", dao_id, year * 100 + quarter, None)
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
    pub fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        roadmap_id: u32,
        name: Vec<u8>,
        priority: u8,
        tags: Option<Vec<u8>>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeDAO(WeteeDaoCall::create_roadmap_task {
            dao_id,
            roadmap_id,
            name,
            priority,
            tags,
        });
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    // DAO 发行货币总量
    pub fn total_issuance(&mut self, dao_id: u64) -> anyhow::Result<u128, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();

        let result: u128 = api
            .get_storage_map("Tokens", "TotalIssuance", dao_id, None)
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
