use codec::{Decode, Encode};
use wetee_dao::QuarterTask;
use wetee_gov::MemmberData;
use wetee_runtime::AccountId;

/// balance information for an account.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
pub struct Quarter {
    // 年
    pub year: u32,
    // 季度
    pub quarter: u32,
    // 任务
    pub tasks: Vec<QuarterTask<AccountId>>,
}

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct WithGov {
    // 1 => gov
    // 2 => sudo
    pub run_type: u8,
    pub amount: u128,
    pub member: MemmberData,
}
