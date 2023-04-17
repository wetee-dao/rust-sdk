use crate::{model::dao::WithGov, Client};

use codec::Compact;
use sp_core::sr25519;
use substrate_api_client::{
    compose_extrinsic, rpc::WsRpcClient, Api, ExtrinsicSigner, GetStorage, PlainTipExtrinsicParams,
    SubmitAndWatchUntilSuccess,
};
pub use wetee_gov::MemmberData;
use wetee_runtime::{AccountId, Hash, Runtime, RuntimeCall, Signature};

use super::base_hander::BaseHander;

// 通过 sudo 或者 gov 执行区块链函数
pub fn run_sudo_or_gov(
    api: Api<
        ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
        WsRpcClient,
        PlainTipExtrinsicParams<Runtime>,
        Runtime,
    >,
    dao_id: u64,
    call: RuntimeCall,
    param: WithGov,
) -> anyhow::Result<(), anyhow::Error> {
    let result = if param.run_type == 1 {
        let xt = compose_extrinsic!(
            &api,
            "WeteeGov",
            "create_propose",
            dao_id,
            MemmberData::<u64>::GLOBAL,
            call,
            Compact(param.amount)
        );
        api.submit_and_watch_extrinsic_until_success(xt, false)
    } else {
        let xt = compose_extrinsic!(&api, "WeteeSudo", "sudo", dao_id, call);
        api.submit_and_watch_extrinsic_until_success(xt, false)
    };

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

/// DAO 治理模块
pub struct WeteeGov {
    pub base: BaseHander,
}

impl WeteeGov {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    // 待开始的投票
    pub fn public_props(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<(u32, Hash, RuntimeCall, MemmberData<u64>, AccountId)>, anyhow::Error>
    {
        let api = self.base.get_client()?;

        let result: Vec<(u32, Hash, RuntimeCall, MemmberData<u64>, AccountId)> = api
            .get_storage_map("WeteeGov", "PublicProps", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    // 获取正在投票的项目
    // pub fn referendum_info(
    //     &mut self,
    //     dao_id: u64,
    //     referendum_index: u32,
    // ) -> anyhow::Result<(Hash, RuntimeCall, MemmberData<u64>, AccountId), anyhow::Error> {
    //     let api = self.base.get_client()?;

    //     let result: (Hash, RuntimeCall, MemmberData<u64>, AccountId) = api
    //         .get_storage_map(
    //             "WeteeGov",
    //             "ReferendumInfoOf",
    //             (dao_id, referendum_index),
    //             None,
    //         )
    //         .unwrap()
    //         .unwrap_or_else(|| {
    //             (
    //                 Hash::default(),
    //                 RuntimeCall::default(),
    //                 MemmberData::<u64>::default(),
    //                 AccountId::default(),
    //             )
    //         });

    //     Ok(result)
    // }
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
