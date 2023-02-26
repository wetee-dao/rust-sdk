use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

/// 公钥类型
pub type PublicFor<P> = <P as sp_core::Pair>::Public;

/// 私钥类型
pub type SeedFor<P> = <P as sp_core::Pair>::Seed;

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyringJSON {
    // 账户地址
    pub address: String,
    // 属性值
    pub encoded: String,
    // 加密方式
    pub encoding: KeyringJSONEncoding,
    // 元数据
    pub meta: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeyringJSONEncoding {
    // 加密方式
    pub content: Vec<String>,
    // 加密类型
    #[serde(rename = "type")]
    pub typex: String,
    // 加密版本
    pub version: String,
}
