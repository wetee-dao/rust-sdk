
use crate::{model::{dao::WithGov, chain::QueryKey}, Client};
use sp_core::{crypto::Ss58Codec, sr25519};
pub use wetee_gov::{MemmberData, Opinion, Referendum, ReferendumStatus};
use wetee_gov::{ReferendumIndex, VoteInfo};
pub use wetee_runtime::Pledge;
use wetee_runtime::{
    AccountId, Balance, BlockNumber, Hash, RuntimeCall, WeteeGovCall, WeteeSudoCall,
};

// 通过 sudo 或者 gov 执行区块链函数
pub async fn run_sudo_or_gov(
    client: &Client,
    from: String,
    dao_id: u64,
    call: RuntimeCall,
    param: WithGov,
) -> anyhow::Result<(), anyhow::Error> {
    let _result = if param.run_type == 1 {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::create_propose { dao_id: dao_id, member_data: param.member, proposal:Box::new(call), value: param.amount });
        return client.send_and_sign(call,from).await;
    };
    let call = RuntimeCall::WeteeSudo(WeteeSudoCall::sudo { dao_id: dao_id, call: Box::new(call)});
    client.send_and_sign(call,from).await
    
}

/// DAO 治理模块
pub struct WeteeGov {
    pub base: Client,
}

impl WeteeGov {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    // 待开始的投票
    pub async fn pending_referendum_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<(u32, Hash, RuntimeCall, MemmberData, AccountId)>, anyhow::Error> {
        let result: Vec<(u32, Hash, RuntimeCall, MemmberData, AccountId)> = self.base.get_storage_map("WeteeGov", "PublicProps", QueryKey::U64Key(dao_id)).await
            .unwrap()
            .unwrap_or_else(|| vec![]);
        Ok(result)
    }

    // 开始一个投票
    pub async fn start_referendum(
        &mut self,
        from: String,
        dao_id: u64,
        propose_id: u32,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::start_referendum { dao_id, propose_id });
        self.base.send_and_sign(call,from).await
    }

    // 获取正在投票的项目
    pub async fn referendum_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<(String, Referendum<BlockNumber, RuntimeCall, Balance>)>, anyhow::Error>
    {
        let results: Vec<(String, Referendum<BlockNumber, RuntimeCall, Balance>)> = self.base.get_storage_double_map_first("WeteeGov", "ReferendumInfoOf", QueryKey::U64Key(dao_id)).await
            .unwrap();

        Ok(results)
    }

    // 投票
    pub async fn vote_for_referendum(
        &mut self,
        from: String,
        dao_id: u64,
        referendum_index: u32,
        vote: u64,
        opinion: bool,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::vote_for_referendum {
            dao_id,
            referendum_index,
            pledge: Pledge::FungToken(vote.into()),
            opinion: if opinion { Opinion::YES } else { Opinion::NO },
        });
        self.base.send_and_sign(call,from).await
    }

    // 获取投票结果
    pub async fn votes_of_user(
        &mut self,
        from: String,
        dao_id: u64,
    ) -> anyhow::Result<
        Vec<VoteInfo<u64, Pledge<Balance>, BlockNumber, Balance, Opinion, ReferendumIndex>>,
        anyhow::Error,
    > {
        let dest = sr25519::Public::from_string(&from).unwrap().into();

        let result: Vec<
            VoteInfo<u64, Pledge<Balance>, BlockNumber, Balance, Opinion, ReferendumIndex>,
        > = self.base.get_storage_map("WeteeGov", "VotesOf", QueryKey::AccountId(dest)).await
            .unwrap()
            .unwrap_or_default();

        Ok(result.into_iter().filter(|x| x.dao_id == dao_id).collect())
    }

    pub async fn run_proposal(
        &mut self,
        from: String,
        dao_id: u64,
        id: u32,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::run_proposal { dao_id, index: id });
        self.base.send_and_sign(call,from).await
    }

    pub async fn unlock(&mut self, from: String, dao_id: u64) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::unlock { dao_id });
        self.base.send_and_sign(call,from).await
    }

    pub async fn set_voting_period(
        &mut self,
        from: String,
        dao_id: u64,
        period: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::set_voting_period { dao_id, period });
        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }
        self.base.send_and_sign(call,from).await
    }

    pub async fn set_runment_period(
        &mut self,
        from: String,
        dao_id: u64,
        period: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let call = RuntimeCall::WeteeGov(WeteeGovCall::set_runment_period { dao_id, period });
        if ext.is_some() {
            return run_sudo_or_gov(&self.base, from, dao_id, call, ext.unwrap()).await;
        }
        self.base.send_and_sign(call,from).await
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
