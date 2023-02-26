use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use subxt::{
    config::{Config, SubstrateConfig},
    ext::sp_core::sr25519::Pair,
    tx::SubstrateExtrinsicParams,
    OnlineClient,
};

#[subxt::subxt(runtime_metadata_path = "./metadata_full.scale")]
pub mod asyoume {}

// 类型
pub type AccountId = <SubstrateConfig as Config>::AccountId;
pub type Moment = u64;
pub const GAS_LIMIT: u64 = 200_000_000_000;

// 区块链runtime配置文件
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AsyouConfig;
impl Config for AsyouConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <SubstrateConfig as Config>::BlockNumber;
    type Hash = <SubstrateConfig as Config>::Hash;
    type Hashing = <SubstrateConfig as Config>::Hashing;
    type AccountId = AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Header = <SubstrateConfig as Config>::Header;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Extrinsic = <SubstrateConfig as Config>::Extrinsic;

    // ExtrinsicParams makes use of the index type, so we need to adjust it
    // too to align with our modified index type, above:
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

// 区块链连接池
pub static API_POOL: Lazy<Mutex<Vec<OnlineClient<AsyouConfig>>>> = Lazy::new(|| Mutex::new(vec![]));

// 账户中心
pub static KERINGS: Lazy<Mutex<HashMap<String, Pair>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

// 获取区块链连接
pub async fn get_api_index(url: String) -> anyhow::Result<u32, anyhow::Error> {
    // 连接区块链
    let mut _api_box = API_POOL.lock().unwrap();

    let api = OnlineClient::<AsyouConfig>::from_url(url.clone()).await?;
    if let Err(_e) = asyoume::validate_codegen(&api) {
        println!("Generated code is not up to date with node we're connected to");
    }
    _api_box.push(api);

    Ok((_api_box.len() - 1) as u32)
}
