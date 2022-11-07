use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use subxt::{
  config::{Config, SubstrateConfig},
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
pub static API_MAP: Lazy<Mutex<Vec<OnlineClient<AsyouConfig>>>> = Lazy::new(|| Mutex::new(vec![]));
pub static API_URL_MAP: Lazy<Mutex<HashMap<&str, u32>>> = Lazy::new(|| {
  let m = HashMap::new();
  Mutex::new(m)
});

// 获取区块链连接
pub async fn get_api_index(url: String) -> u32 {
  // 连接区块链
  let _api_box = API_MAP.lock().unwrap();
  let _api_url_box = API_URL_MAP.lock().unwrap();

  // // 查到连接直接返回index
  // match _api_url_box.get(url.as_str()) {
  //   Some(v) => v,
  // };

  let api = OnlineClient::<AsyouConfig>::from_url(url.clone())
    .await
    .unwrap();
  if let Err(_e) = asyoume::validate_codegen(&api) {
    println!("Generated code is not up to date with node we're connected to");
  }
  API_MAP.lock().unwrap().push(api);
  let curl = url.clone();
  // API_URL_MAP.lock().unwrap().push(curl.as_str());

  API_MAP.lock().unwrap().len() as u32 - 1
}
