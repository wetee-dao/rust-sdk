// use codec::Decode;
// use core::marker::PhantomData;
// use sp_core::{sr25519::Pair, storage::StorageKey, Pair as TraitPair};
// use std::collections::HashMap;
// use std::str::FromStr;
// use std::sync::Arc;
// use subxt::{system::System, DefaultNodeRuntime, Error as xtError, PairSigner, UncheckedExtrinsic};

// use std::error::Error;

// use super::super::client::Client;
// use super::super::error_types::Error as RuntimeError;
// use super::super::model::{art::*, feeless::*, timestamp::*};

// /// 艺术品
// pub struct Art {
//   // 连接
//   client: Arc<Client>,
//   // 无费模式
//   feeless: bool,
// }

// impl Art {
//   pub fn new(c: Arc<Client>) -> Self {
//     Art {
//       client: c,
//       feeless: false,
//     }
//   }

//   pub fn feeless(mut self) -> Self {
//     self.feeless = true;
//     self
//   }

//   // 添加艺术品
//   pub async fn register_art(&self, call_args: RegisterArtData) -> Result<String, Box<dyn Error>> {
//     let signer =
//       Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
//     let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);

//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new()
//       .set_url(self.client.uri.as_str())
//       .skip_type_sizes_check()
//       .build()
//       .await?;

//     // 构造请求参数
//     let mut props: Vec<ArtProperty> = Vec::new();
//     for v in call_args.props {
//       props.push(ArtProperty {
//         name: v.name.clone().into_bytes(),
//         value: v.value.clone().into_bytes(),
//       });
//     }
//     let mut entity_ids: Vec<ArtStr> = Vec::new();
//     for v in call_args.entity_ids {
//       entity_ids.push(v.clone().into_bytes());
//     }

//     // 构造请求
//     let owner = sp_runtime::AccountId32::from_str(&call_args.owner)?;
//     let art_call = RegisterArtCall::<DefaultNodeRuntime> {
//       id: call_args.id.clone().into_bytes(),
//       props: props,
//       owner: owner,
//       entity_ids: entity_ids,
//       hash_method: call_args.hash_method.clone().into_bytes(),
//       hash: call_args.hash.clone().into_bytes(),
//       dna_method: call_args.dna_method.clone().into_bytes(),
//       dna: call_args.dna.clone().into_bytes(),
//       _runtime: PhantomData,
//     };

//     let extrinsic: UncheckedExtrinsic<DefaultNodeRuntime>;
//     #[allow(unused_assignments)]
//     let mut block_hash = String::from("");
//     if !self.feeless {
//       // 签名
//       extrinsic = client.create_signed(art_call, &signer).await?;
//     } else {
//       let report_proposal = client.encode(art_call)?;
//       let feeless_call = FeelessCall::<DefaultNodeRuntime> {
//         call: &report_proposal,
//         _runtime: PhantomData,
//       };
//       // 签名
//       extrinsic = client.create_signed(feeless_call, &signer).await?;
//     }

//     // 构造错误接受
//     // let mut decoder = client.events_decoder::<RegisterReportCall<DefaultNodeRuntime>>();
//     // decoder.with_pacs_deposit();
//     // let event_result = client.rpc.submit_and_watch_extrinsic(extrinsic, decoder).await;
//     // 提交请求
//     let event_result = client.submit_and_watch_extrinsic(extrinsic).await;
//     // let event_result = client.register_report_and_watch(&signer, call_args.id.clone().into_bytes(), Some(props)).await;
//     match event_result {
//       Ok(s) => {
//         block_hash = "0x".to_string() + &hex::encode(&s.block[..].to_vec());
//       }
//       Err(xtError::Runtime(e)) => {
//         let emain = e.to_string();
//         let estr: Vec<&str> = emain
//           .trim_start_matches("Runtime module error:")
//           .split(" from ")
//           .collect();
//         return Err(estr[0].trim_start_matches(" ").into());
//       }
//       Err(e) => return Err(("调用错误:".to_owned() + &(e.to_string())).into()),
//     };
//     Ok(block_hash)
//   }

//   // 获取模块数据
//   pub async fn get_artkey(&self, id: &str) -> Result<String, Box<dyn Error>> {
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new()
//       .set_url(self.client.uri.as_str())
//       .skip_type_sizes_check()
//       .build()
//       .await?;

//     // 获取key
//     let metadata = client.metadata();
//     let key = metadata
//       .module("Art")?
//       .storage("Arts")?
//       .map()?
//       .key(&id.as_bytes().to_vec());
//     let realkey = "0x".to_string() + &hex::encode(key.clone().0);
//     Ok(realkey)
//   }

//   // 获取模块数据
//   pub async fn get_entitykey(&self, id: &str) -> Result<String, Box<dyn Error>> {
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new()
//       .set_url(self.client.uri.as_str())
//       .skip_type_sizes_check()
//       .build()
//       .await?;

//     // 获取key
//     let metadata = client.metadata();
//     let key = metadata
//       .module("Art")?
//       .storage("Entitys")?
//       .map()?
//       .key(&id.as_bytes().to_vec());
//     let realkey = "0x".to_string() + &hex::encode(key.clone().0);
//     Ok(realkey)
//   }
// }
