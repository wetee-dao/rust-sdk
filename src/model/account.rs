
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, collections::HashMap};

// {
// 	"address": "5EHixZR7DN2xAUZpA27PvmgX1x26GuWa4DUmUKE5iNsdtkLz",
// 	"encoded": "0xf5372943bf94e72faaa19c8ea5032cb558840b754c13dd473bc7e77efe689f95caff2cae984ab599583c931a89df18e6ef6f96b0c40b0ac7ffd1d884ac7dd0a77951be9be9b5809ab77320eab3e9d8ec830c78a3084f71219fc9d396d11d21788bc12ee0a75fefac254a23989710d006150677bfd59654a45527a752749cda3ca3ec55df85a8a7fb3ffd70cd1949faf87de71c14cbe1ce317c661afe75",
// 	"encoding": {
// 		"content": ["pkcs8", "sr25519"],
// 		"type": "xsalsa20-poly1305",
// 		"version": "2"
// 	},
// 	"meta": {
// 		"name": "门系统"
// 	}
// }

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeringJSON {
  // 账户地址
  pub address: String,
  // 属性值
  pub encoded: String,
  // 加密方式
  pub encoding: KeringJSONEncoding,
  // 元数据
  pub meta: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct KeringJSONEncoding{
  // 加密方式
  pub content: Vec<String>,
  // 加密类型
  #[serde(rename = "type")]
  pub typex: String,
  // 加密版本
  pub version: String
}
