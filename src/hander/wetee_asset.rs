use crate::{account, model::account::AssetAccountData, chain::API_POOL_NEW};

use super::super::client::Client;
use super::base_hander::BaseHander;
use wetee_runtime::{Runtime, RuntimeCall, Signature, WeteeAssetsCall};

use sp_core::{crypto::Ss58Codec, sr25519};
use sp_runtime::MultiAddress;
use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// 账户
pub struct WeteeAsset {
    pub base: BaseHander,
}

impl WeteeAsset {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub fn balance(
        &mut self,
        dao_id: u64,
        address: String,
    ) -> anyhow::Result<AssetAccountData<u128>, anyhow::Error> {
        let pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get(self.base.client.index).unwrap();

        let v = sr25519::Public::from_string(&address).unwrap();
        let balance: AssetAccountData<u128> = api
            .get_storage_double_map("Tokens", "Accounts", v, dao_id, None)
            .unwrap()
            .unwrap_or_default();

        Ok(balance)
    }

    pub fn create_asset(
        &mut self,
        from: String,
        dao_id: u64,
        meta_name: String,
        meta_symbol: String,
        amount: u128,
        init_dao_asset: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::create_asset {
            dao_id,
            metadata: wetee_assets::DaoAssetMeta {
                name: meta_name.into(),
                symbol: meta_symbol.into(),
                decimals: 4,
            },
            amount,
            init_dao_asset,
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

    pub fn set_existenial_deposit(
        &mut self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_existenial_deposit {
            dao_id,
            existenial_deposit: amount,
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

    pub fn set_metadata(
        &mut self,
        from: String,
        dao_id: u64,
        metadata: wetee_assets::DaoAssetMeta,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::set_metadata { dao_id, metadata });
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

    pub fn burn(
        &mut self,
        from: String,
        dao_id: u64,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::burn { dao_id, amount });
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

    pub fn transfer(
        &mut self,
        from: String,
        dao_id: u64,
        to: String,
        amount: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let dest = sr25519::Public::from_string(&to).unwrap();
        let signer_nonce = api.get_nonce().unwrap();
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::transfer {
            dao_id,
            amount,
            dest: MultiAddress::Id(dest.into()),
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

    pub fn join_request(
        &mut self,
        from: String,
        dao_id: u64,
        share_expect: u32,
        existenial_deposit: u128,
    ) -> anyhow::Result<(), anyhow::Error> {
        let mut pool = API_POOL_NEW.lock().unwrap();
        let api =  pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        // 构建请求
        let call = RuntimeCall::WeteeAsset(WeteeAssetsCall::join_request {
            dao_id,
            share_expect,
            existenial_deposit,
        });
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
