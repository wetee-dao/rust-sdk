use crate::{account, chain::API_CLIENT_POOL, model::account::AssetAccountData};

use super::super::client::Client;

use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use sp_runtime::MultiAddress;
use substrate_api_client::{
    extrinsic::BalancesExtrinsics, ExtrinsicSigner, GetAccountInformation,
    SubmitAndWatchUntilSuccess,
};
use wetee_runtime::{Runtime, Signature};

/// 账户
pub struct Balance {
    pub base: Client,
}

impl Balance {
    pub fn new(c: Client) -> Self {
        Self { base: c }
    }

    pub fn balance(
        &mut self,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        let v = sr25519::Public::from_string(&address).unwrap();
        let balance = api.get_account_data(&v.into()).unwrap().unwrap_or_default();

        Ok(AssetAccountData {
            free: balance.free.try_into().unwrap(),
            frozen: balance.fee_frozen.try_into().unwrap(),
            reserved: balance.reserved.try_into().unwrap(),
            // balance.misc_frozen,
        })
    }

    pub fn transfer(
        &mut self,
        from: String,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone()).unwrap();
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构造请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let xt = api.balance_transfer(MultiAddress::Id(dest.into()), amount);
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

    pub fn init_from_pair(
        &mut self,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let alice: sr25519::Pair = Pair::from_string(
            "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a",
            None,
        )
        .unwrap();
        println!("signer account: {}", alice.public().to_ss58check());

        // Initialize api and set the signer (sender) that is used to sign the extrinsics.
        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(alice.clone()));

        // 构造请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let xt = api.balance_transfer(MultiAddress::Id(dest.into()), amount);
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
