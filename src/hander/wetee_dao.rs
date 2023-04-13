use crate::{account, model::dao::Quarter};

use super::super::client::Client;
use super::base_hander::BaseHander;
use wetee_dao::QuarterTask;
use wetee_runtime::{AccountId, Runtime, RuntimeCall, Signature, WeteeDaoCall};

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

    pub fn next_dao_id(&mut self) -> anyhow::Result<u64, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: u64 = api
            .get_storage_value("WeteeDAO", "NextDaoId", None)
            .unwrap()
            .unwrap_or_else(|| 5000);

        Ok(result)
    }

    pub fn create_dao(
        &mut self,
        from: String,
        name: String,
        purpose: String,
        meta_data: String,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

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

    pub fn roadmap_list(
        &mut self,
        dao_id: u64,
        year: u32,
    ) -> anyhow::Result<Vec<Quarter>, anyhow::Error> {
        let api = self.base.get_client()?;
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

    pub fn create_task(
        &mut self,
        from: String,
        dao_id: u64,
        roadmap_id: u32,
        name: Vec<u8>,
        priority: u8,
        description: Vec<u8>,
        tags: Option<Vec<u8>>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeDAO(WeteeDaoCall::create_roadmap_task {
            dao_id,
            roadmap_id,
            name,
            priority,
            description,
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
