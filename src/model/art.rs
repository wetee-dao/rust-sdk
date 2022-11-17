use super::super::chain::*;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// 类型
pub type ArtStr = Vec<u8>;

// // 存证属性
// #[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
// pub struct ArtProperty {
//   // 属性名
//   pub name: ArtStr,
//   // 属性值
//   pub value: ArtStr,
// }

// // 存证请求参数
// #[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
// pub struct RegisterArtData {
//   pub id: String,
//   pub owner: String,
//   pub entity_ids: Vec<String>,
//   pub props: Vec<ArtPropertyData>,
//   pub hash_method: String,
//   pub hash: String,
//   pub dna_method: String,
//   pub dna: String,
// }
// #[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
// pub struct ArtPropertyData {
//   // 属性名
//   pub name: String,
//   // 属性值
//   pub value: String,
// }

// // 存证请求函数
// #[derive(Clone, Debug, Eq, PartialEq, Encode)]
// pub struct RegisterArtCall {
//   pub id: ArtStr,
//   pub owner: AccountId,
//   pub entity_ids: Vec<ArtStr>,
//   pub props: Vec<ArtProperty>,
//   pub hash_method: ArtStr,
//   pub hash: ArtStr,
//   pub dna_method: ArtStr,
//   pub dna: ArtStr,
// }
