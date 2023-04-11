use crate::account;

use super::super::client::Client;
use super::base_hander::BaseHander;
use wetee_runtime::{Runtime, RuntimeCall, Signature, WeteeDaoCall};

use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// 账户
pub struct WeteeDAO {
    pub base: BaseHander,
}

impl WeteeDAO {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub fn nex_dao_id(&mut self) -> anyhow::Result<u64, anyhow::Error> {
        let api = self.base.get_client()?;

        // 构建请求
        let result: u64 = api
            .get_storage_value("WeteeDAO", "NextDaoId", None)
            .unwrap()
            .unwrap_or_else(|| 1);

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
