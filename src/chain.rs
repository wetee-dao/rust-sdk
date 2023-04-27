use once_cell::sync::Lazy;
use sp_core::sr25519::{self, Pair};
use std::collections::HashMap;
use std::sync::Mutex;
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, ExtrinsicSigner, PlainTipExtrinsicParams};
use wetee_runtime::{Runtime, Signature};

// 账户中心
pub static KERINGS: Lazy<Mutex<HashMap<String, Pair>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub const UNIT: u64 = 1_000_000_000_000;

// 全局区块链连接
pub static API_CLIENT_POOL: Lazy<
    Mutex<
        Vec<
            Api<
                ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
                WsRpcClient,
                PlainTipExtrinsicParams<Runtime>,
                Runtime,
            >,
        >,
    >,
> = Lazy::new(|| Mutex::new(vec![]));

// 获取区块链连接
pub fn get_api(url: String) -> anyhow::Result<usize, anyhow::Error> {
    // 连接区块链
    let mut _api_box = API_CLIENT_POOL.lock().unwrap();

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

    _api_box.push(api.unwrap());

    Ok(_api_box.len() - 1)
}
