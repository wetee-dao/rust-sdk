use crate::account;

use super::super::client::Client;
use super::base_hander::BaseHander;
use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::MultiAddress;
use substrate_api_client::{
    extrinsic::BalancesExtrinsics, ExtrinsicSigner, GetAccountInformation,
    SubmitAndWatchUntilSuccess,
};
use wetee_runtime::{Runtime, Signature};

/// 账户
pub struct Balance {
    pub base: BaseHander,
}

impl Balance {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub fn balance(
        &mut self,
        address: String,
    ) -> anyhow::Result<(u128, u128, u128, u128), anyhow::Error> {
        let api = self.base.get_client()?;

        let v = sr25519::Public::from_string(&address).unwrap();
        let balance = api.get_account_data(&v.into()).unwrap().unwrap_or_default();

        println!("[+] balance's Free Balance is is {}\n", balance.free);
        println!("{}", balance.free);

        Ok((
            balance.free,
            balance.fee_frozen,
            balance.reserved,
            balance.misc_frozen,
        ))
    }

    pub fn transfer(
        &mut self,
        from: String,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut api = self.base.get_client()?;

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
}
