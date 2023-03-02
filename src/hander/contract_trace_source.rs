use codec::Encode;
use ink_env::call::{ExecutionInput, Selector};
use sp_runtime::{
    traits::{BlakeTwo256, Hash},
    MultiAddress,
};
use std::convert::TryInto;
use std::str::FromStr;
use subxt::tx::{PairSigner, TxStatus::*};

use super::super::account::*;
use super::super::chain::*;
use super::super::client::Client;
use super::super::model::contract_trace_source::*;
use super::base_hander::BaseHander;

/// 智能合约
pub struct ContractTraceSource {
    pub base: BaseHander,
}

impl ContractTraceSource {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub async fn trace_init(
        &mut self,
        constract_id: &str,
        p: CTST,
        first_record: CTSTRecord,
        from: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 获取区块链接口
        let api = self.base.get_client().await?;

        // 构建创建者
        let signer = get_from_address(from).expect("Could not obtain stash signer pair");
        let signer = PairSigner::new(signer);

        // 初始化合约
        let contract_address = sp_runtime::AccountId32::from_str(constract_id)?;

        // 构建参数
        let meta = base64::encode(first_record.meta);
        let call_data = ExecutionInput::new(Selector::new(
            BlakeTwo256::hash("init".as_bytes())[0..4].try_into()?,
        ))
        .push_arg(&p.id)
        .push_arg(meta);

        // 构建请求
        let call_tx = wetee_chain::tx().contracts().call(
            MultiAddress::Id(contract_address.clone()),
            0,         // value
            GAS_LIMIT, // gas_limit
            None,      // storage_deposit_limit
            call_data.encode(),
        );

        // 执行智能合约
        let mut call_progress = api
            .tx()
            .sign_and_submit_then_watch_default(&call_tx, &signer)
            .await?;

        // 获取执行结果
        while let Some(ev) = call_progress.next().await {
            let ev = ev?;
            // 执行中
            if let InBlock(details) = ev {
                println!(
                    "Transaction {:?} made it into block {:?}",
                    details.extrinsic_hash(),
                    details.block_hash()
                );
            } else if let Finalized(details) = ev {
                println!(
                    "Transaction {:?} is finalized in block {:?}",
                    details.extrinsic_hash(),
                    details.block_hash()
                );
                let events = details.wait_for_success().await?;
                let call_event = events.find_first::<wetee::system::events::ExtrinsicSuccess>()?;
                if let Some(event) = call_event {
                    println!("Balance transfer success: {event:?}");
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            } else {
                println!("Current transaction status: {:?}", ev);
            }
        }
        Ok(())
    }
}
