use once_cell::sync::Lazy;
use sp_core::sr25519::Pair;
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
