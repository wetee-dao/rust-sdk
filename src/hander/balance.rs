use std::str::FromStr;

use sp_keyring::AccountKeyring;

use crate::account;

use super::super::client::Client;
use super::base_hander::BaseHander;
use node_template_runtime::{Runtime, Signature};
use sp_core::{
    crypto::{Pair, Ss58Codec},
    sr25519,
};
use sp_runtime::{MultiAddress, MultiSignature};
use substrate_api_client::{
    extrinsic::BalancesExtrinsics, rpc::WsRpcClient, Api, ExtrinsicSigner, GetAccountInformation,
    PlainTipExtrinsicParams, SubmitAndWatch, XtStatus,
};
type RunBalance = <Runtime as pallet_balances::Config>::Balance;
use pallet_balances::AccountData as GenericAccountData;
type AccountData = GenericAccountData<RunBalance>;

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

    pub async fn amount(
        &mut self,
        address: String,
    ) -> anyhow::Result<(u128, u128, u128, u128), anyhow::Error> {
        let url = self.base.client.get_url().unwrap();

        // 获取区块链接口
        let client = WsRpcClient::new(&url).unwrap();
        let api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            WsRpcClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();
        // let alice: sr25519::Pair = Pair::from_string(
        //     "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a",
        //     None,
        // )
        // .unwrap();
        // api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(alice.clone()));

        let v = sr25519::Public::from_ss58check(&address).unwrap();

        let balance = api
            .get_account_data(&v.into())
            .unwrap()
            .unwrap_or_default()
            .free;

        println!("[+] balance's Free Balance is is {}\n", balance);
        println!("{}", balance);

        Ok((balance, 0, 0, 0))
    }

    pub async fn transfer(
        &mut self,
        from: String,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let url = self.base.client.get_url().unwrap();

        // 获取区块链接口
        let client = WsRpcClient::new(&url).unwrap();
        let mut api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            WsRpcClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();

        let from_pair = account::get_from_address(from.clone()).unwrap();
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));
        let dest = sr25519::Public::from_ss58check(&to).unwrap();

        let xt = api.balance_transfer(MultiAddress::Id(dest.into()), amount);
        println!("[+] Composed extrinsic: {:?}\n", xt);

        Ok(())
    }
}
