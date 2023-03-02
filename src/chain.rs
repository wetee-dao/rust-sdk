use once_cell::sync::Lazy;
use sp_core::sr25519::Pair;
use std::collections::HashMap;
use std::sync::Mutex;
use subxt::{
    config::{Config, SubstrateConfig},
    tx::SubstrateExtrinsicParams,
};

#[subxt::subxt(runtime_metadata_path = "./metadata_full.scale")]
pub mod wetee_chain {}

// 类型
pub type AccountId = <SubstrateConfig as Config>::AccountId;
pub type Moment = u64;
pub const GAS_LIMIT: u64 = 200_000_000_000;

// 区块链runtime配置文件
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WeteeConfig;
impl Config for WeteeConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u32;
    type BlockNumber = <SubstrateConfig as Config>::BlockNumber;
    type Hash = <SubstrateConfig as Config>::Hash;
    type Hashing = <SubstrateConfig as Config>::Hashing;
    type AccountId = AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Header = <SubstrateConfig as Config>::Header;
    type Signature = <SubstrateConfig as Config>::Signature;
    // ExtrinsicParams makes use of the index type, so we need to adjust it
    // too to align with our modified index type, above:
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

// 区块链连接池
pub static API_POOL: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

// 账户中心
pub static KERINGS: Lazy<Mutex<HashMap<String, Pair>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

// 获取区块链连接
pub fn get_api_index(url: String) -> anyhow::Result<u32, anyhow::Error> {
    // 连接区块链
    let mut _api_box = API_POOL.lock().unwrap();

    _api_box.push(url);

    Ok((_api_box.len() - 1) as u32)
}
