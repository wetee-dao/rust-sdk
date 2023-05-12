use crate::{account::add_pair, model::{account::AssetAccountData, chain::QueryKey}};

use super::super::client::Client;

use pallet_balances::AccountData;
use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use sp_runtime::MultiAddress;

use wetee_runtime::{RuntimeCall};

/// 账户
pub struct Balance {
    pub base: Client,
}

impl Balance {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    pub async fn balance(
        &mut self,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let id = sr25519::Public::from_string(&address).unwrap().into();
        let balance:AccountData<u128> = self.base.get_storage_map("Balances", "Account", QueryKey::AccountId(id)).await.unwrap().unwrap_or_default();
        Ok(AssetAccountData {
            free: balance.free.try_into().unwrap(),
            frozen: balance.fee_frozen.try_into().unwrap(),
            reserved: balance.reserved.try_into().unwrap(),
            // balance.misc_frozen,
        })
    }

    pub async fn transfer(
        &mut self,
        from: String,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构造请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest: MultiAddress::Id(dest.into()), value: amount });
        self.base.send_and_sign(call,from).await
    }

    pub async fn init_from_pair(
        &mut self,
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
