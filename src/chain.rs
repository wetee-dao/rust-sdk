use once_cell::sync::Lazy;
use sp_core::sr25519::{Pair};
use std::collections::HashMap;
use std::sync::Mutex;

// 账户中心
pub static KERINGS: Lazy<Mutex<HashMap<String, Pair>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub const UNIT: u64 = 1_000_000_000_000;
