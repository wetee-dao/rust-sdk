use crate::{account::add_pair, model::{account::AssetAccountData, chain::QueryKey}};

use super::super::client::Client;

use codec::{Encode, Decode, MaxEncodedLen};
// use pallet_balances::AccountData;
use sp_core::{crypto::Ss58Codec, sr25519, Pair, RuntimeDebug};
use sp_runtime::MultiAddress;

use substrate_api_client::ac_primitives::AccountInfo;
use wetee_runtime::{RuntimeCall, Nonce};

/// 账户
pub struct Balance {
    pub base: Client,
}

impl Balance {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    /// 资产
    pub async fn balance(
        & self,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let id = sr25519::Public::from_string(&address).unwrap().into();
        let account:AccountInfo<
           Nonce,
           AccountData<u128>,
        > = self.base.get_storage_map("System", "Account", QueryKey::AccountId(id)).await.unwrap().unwrap_or_default();
        Ok(AssetAccountData {
            free: account.data.free,
            frozen: account.data.frozen,
            reserved: account.data.reserved,
            // balance.misc_frozen,
        })
    }

    /// 转账
    pub async fn transfer(
        & self,
        from: String,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构造请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest: MultiAddress::Id(dest.into()), value: amount });
        self.base.send_and_sign(call,from).await
    }

    /// 从 dev pair 转账
    pub async fn init_from_pair(
        & self,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let alice: sr25519::Pair = Pair::from_string(
            "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a",
            None,
        )
        .unwrap();
        
        let (address,_) =  add_pair(alice).unwrap();
        println!("signer account: {}",address);

        // 构造请求
        let dest = sr25519::Public::from_string(&to).unwrap();

        let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest: MultiAddress::Id(dest.into()), value: amount });
        self.base.send_and_sign(call,address).await
    }
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen)]
pub struct AccountData<Balance> {
	/// Non-reserved part of the balance which the account holder may be able to control.
	///
	/// This is the only balance that matters in terms of most operations on tokens.
	pub free: Balance,
	/// Balance which is has active holds on it and may not be used at all.
	///
	/// This is the sum of all individual holds together with any sums still under the (deprecated)
	/// reserves API.
	pub reserved: Balance,
	/// The amount that `free` may not drop below when reducing the balance, except for actions
	/// where the account owner cannot reasonably benefit from thr balance reduction, such as
	/// slashing.
	pub frozen: Balance,
}
