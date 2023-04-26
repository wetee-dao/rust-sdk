use once_cell::sync::Lazy;
use sp_core::sr25519::{Pair, self};
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, ExtrinsicSigner, PlainTipExtrinsicParams};
use wetee_runtime::{Signature, Runtime};
use std::collections::HashMap;
use std::sync::Mutex;

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


// 全局区块链连接
pub static API_POOL_NEW: Lazy<Mutex<Vec<
    Api<
        ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
        WsRpcClient,
        PlainTipExtrinsicParams<Runtime>,
        Runtime,
    >
>>> = Lazy::new(|| Mutex::new(vec![]));

// 获取区块链连接
pub fn get_api(url: String) -> anyhow::Result<usize, anyhow::Error>  {
    // 连接区块链
    let mut _api_box = API_POOL_NEW.lock().unwrap();
    // let mut _api_url_box = API_URL_NEW.lock().unwrap();

    // 获取区块链接口
    let client = WsRpcClient::new(&url).unwrap();
    let api = Api::<
        ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
        WsRpcClient,
        PlainTipExtrinsicParams<Runtime>,
        Runtime,
    >::new(client);
    if api.is_err() {
        return Err(anyhow::anyhow!("url is not ok"));
    }

    println!("cleint ok");

    _api_box.push(api.unwrap());

    Ok(_api_box.len() - 1)
  }