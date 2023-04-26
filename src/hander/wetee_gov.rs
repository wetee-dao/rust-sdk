use super::base_hander::BaseHander;
use crate::{account, chain::API_CLIENT_POOL, model::dao::WithGov, Client};
use codec::Compact;
use sp_core::{crypto::Ss58Codec, sr25519};
use substrate_api_client::{
    compose_extrinsic, ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess,
};
pub use wetee_gov::{MemmberData, Opinion, Referendum, ReferendumStatus};
use wetee_gov::{ReferendumIndex, VoteInfo};
pub use wetee_runtime::Pledge;
use wetee_runtime::{
    AccountId, Balance, BlockNumber, Hash, Runtime, RuntimeCall, Signature, WeteeGovCall,
};

// 通过 sudo 或者 gov 执行区块链函数
pub fn run_sudo_or_gov(
    client_id: usize,
    from: String,
    dao_id: u64,
    call: RuntimeCall,
    param: WithGov,
) -> anyhow::Result<(), anyhow::Error> {
    let mut pool = API_CLIENT_POOL.lock().unwrap();
    let api = pool.get_mut(client_id).unwrap();

    let from_pair = account::get_from_address(from.clone())?;
    api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

    let result = if param.run_type == 1 {
        let xt = compose_extrinsic!(
            &api,
            "WeteeGov",
            "create_propose",
            dao_id,
            param.member,
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
    pub fn pending_referendum_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<(u32, Hash, RuntimeCall, MemmberData, AccountId)>, anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        let result: Vec<(u32, Hash, RuntimeCall, MemmberData, AccountId)> = api
            .get_storage_map("WeteeGov", "PublicProps", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);
        Ok(result)
    }

    // 开始一个投票
    pub fn start_referendum(
        &mut self,
        from: String,
        dao_id: u64,
        propose_id: u32,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeGov(WeteeGovCall::start_referendum { dao_id, propose_id });
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

    // 获取正在投票的项目
    pub fn referendum_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<(String, Referendum<BlockNumber, RuntimeCall, Balance>)>, anyhow::Error>
    {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        let key = api
            .get_storage_double_map_key_prefix("WeteeGov", "ReferendumInfoOf", dao_id)
            .unwrap();

        let storage_keys = api
            .get_storage_keys_paged(Some(key), 1000, None, None)
            .unwrap();

        let mut results: Vec<(String, Referendum<BlockNumber, RuntimeCall, Balance>)> = vec![];
        for storage_key in storage_keys.iter() {
            let storage_data: Referendum<BlockNumber, RuntimeCall, Balance> = api
                .get_storage_by_key_hash(storage_key.clone(), None)
                .unwrap()
                .unwrap();
            let hash = "0x".to_owned() + &hex::encode(storage_key.clone().0);
            results.push((hash, storage_data));
        }

        Ok(results)
    }

    // 投票
    pub fn vote_for_referendum(
        &mut self,
        from: String,
        dao_id: u64,
        referendum_index: u32,
        vote: u64,
        opinion: bool,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeGov(WeteeGovCall::vote_for_referendum {
            dao_id,
            referendum_index,
            pledge: Pledge::FungToken(vote.into()),
            opinion: if opinion { Opinion::YES } else { Opinion::NO },
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

    // 获取投票结果
    pub fn votes_of_user(
        &mut self,
        from: String,
        dao_id: u64,
    ) -> anyhow::Result<
        Vec<VoteInfo<u64, Pledge<Balance>, BlockNumber, Balance, Opinion, ReferendumIndex>>,
        anyhow::Error,
    > {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        let dest = sr25519::Public::from_string(&from).unwrap();

        let result: Vec<
            VoteInfo<u64, Pledge<Balance>, BlockNumber, Balance, Opinion, ReferendumIndex>,
        > = api
            .get_storage_map("WeteeGov", "VotesOf", dest, None)
            .unwrap()
            .unwrap_or_default();

        Ok(result.into_iter().filter(|x| x.dao_id == dao_id).collect())
    }

    pub fn run_proposal(
        &mut self,
        from: String,
        dao_id: u64,
        id: u32,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeGov(WeteeGovCall::run_proposal { dao_id, index: id });
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

    pub fn unlock(&mut self, from: String, dao_id: u64) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeGov(WeteeGovCall::unlock { dao_id });
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

    pub fn set_voting_period(
        &mut self,
        from: String,
        dao_id: u64,
        period: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::set_voting_period { dao_id, period });
        if ext.is_some() {
            return run_sudo_or_gov(self.base.client.index, from, dao_id, call, ext.unwrap());
        }

        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();

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

    pub fn set_runment_period(
        &mut self,
        from: String,
        dao_id: u64,
        period: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::set_runment_period { dao_id, period });
        if ext.is_some() {
            return run_sudo_or_gov(self.base.client.index, from, dao_id, call, ext.unwrap());
        }

        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();

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
