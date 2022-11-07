// use std::sync::Arc;
// use core::marker::PhantomData;
// use std::collections::HashMap;
// use subxt::{
//   UncheckedExtrinsic,
//   PairSigner, DefaultNodeRuntime, system::{System},
//   Error as xtError,
// };
// use codec::{Decode};
// use sp_core::{sr25519::Pair, Pair as TraitPair, storage::StorageKey};

// use std::error::Error;

// use super::super::error_types::Error as RuntimeError;
// use super::super::model::{
//   pacs_deposit::*,
//   timestamp::*,
//   feeless::*,
// };
// use super::super::client::Client;

// /// 医疗影像存证
// pub struct PacsDeposit {
//   // 连接
//   client: Arc<Client>,
//   // 无费模式
//   feeless: bool,
// }

// impl PacsDeposit {
//   pub fn new(c: Arc<Client>) -> Self {
//     PacsDeposit {
//       client: c,
//       feeless: false,
//     }
//   }

//   pub fn feeless(mut self) -> Self {
//     self.feeless = true;
//     self
//   }

//   // 生成报告
//   pub async fn register_report(&self, call_args: ReportData) -> Result<String, Box<dyn Error>>{
//     let signer = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
//     let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).
//       // register_type_size::<sp_core::OpaquePeerId>("PeerId").
//       skip_type_sizes_check().
//       build().await?;
//     // 构造请求参数
//     let mut props:Vec<ReportProperty> = Vec::new();
//     for v in call_args.props {
//       props.push(ReportProperty::new(v.name.clone().into_bytes(),v.value.clone().into_bytes()));
//     }

//     // 构造请求
//     let report_call = RegisterReportCall::<DefaultNodeRuntime> {
//       id: call_args.id.clone().into_bytes(),
//       props: Some(props),
//       _runtime: PhantomData,
//     };

//     let extrinsic: UncheckedExtrinsic<DefaultNodeRuntime>;
//     #[allow(unused_assignments)]
//     let mut block_hash = String::from("");
//     if !self.feeless {
//       // 签名
//       extrinsic = client.create_signed(report_call, &signer).await?;
//     } else {
//       let report_proposal = client.encode(report_call)?;

//       let feeless_call = FeelessCall::<DefaultNodeRuntime> {
//         call: &report_proposal,
//         _runtime: PhantomData,
//       };
//       // 签名
//       extrinsic = client.create_signed(feeless_call, &signer).await?;
//     }

//     // // 构造错误接受
//     // let mut decoder = client.events_decoder::<RegisterReportCall<DefaultNodeRuntime>>();
//     // decoder.with_pacs_deposit();
//     // let event_result = client.rpc.submit_and_watch_extrinsic(extrinsic, decoder).await;
//     // 提交请求
//     let event_result = client.submit_and_watch_extrinsic(extrinsic).await;
//     // let event_result = client.register_report_and_watch(&signer, call_args.id.clone().into_bytes(), Some(props)).await;
//     match event_result {
//       Ok(s) => {
//         block_hash = "0x".to_string()+&hex::encode(&s.block[..].to_vec());
//       },
//       Err(xtError::Runtime(e)) => {
//         let emain = e.to_string();
//         let estr: Vec<&str> = emain.trim_start_matches("Runtime module error:").split(" from ").collect();
//         return Err(estr[0].trim_start_matches(" ").into())
//       },
//       Err(e) => return Err(("调用错误:".to_owned()+&(e.to_string())).into()),
//     };
//     Ok(block_hash)
//   }

//   // 报告列表
//   pub async fn report_list(&self, count: u32, start_hash: Option<String>) -> Result<Vec<ReportDetail>, Box<dyn Error>>{
//     // let signer = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
//     // let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).
//       skip_type_sizes_check().
//       build().await?;
//     // 查询数据key
//     let mut start_key: Option<StorageKey> = None;
//     if start_hash != None {
//       start_key = Some(StorageKey(
//         hex::decode(str::replace(&start_hash.unwrap(), "0x", "")).unwrap(),
//       ));
//     }
//     let keys = client.fetch_keys::<ReportsStore<_>>(count, start_key, None).await.unwrap();
//     let block_hash = client.block_hash(Some(1.into())).await?;

//     // 查询数据
//     let datas = client.query_storage(keys.clone(),block_hash.unwrap(),None).await.unwrap();

//     // 构建返回数据
//     let mut report_map = HashMap::<String,ReportDetail>::new();
//     let mut hashs: Vec<String> = Vec::new();
//     for key in keys {
//       let _hash_key = "0x".to_string()+&hex::encode(&key.0);
//       report_map.insert(_hash_key.clone(), ReportDetail{
//         key: _hash_key.clone(),
//         curent_value: None,
//         history: Vec::new(),
//       });
//       hashs.push(_hash_key.clone())
//     }

//     // 构建查询数据
//     for change_set in datas {
//       for (k, v) in change_set.changes {
//         if v != None {
//           let vdata = v.unwrap();
//           // 解码数据
//           let r: Report<
//             <DefaultNodeRuntime as System>::AccountId,
//             <DefaultNodeRuntime as Timestamp>::Moment
//           > = Decode::decode(&mut &vdata.0[..]).unwrap();

//           // 转换报告属性数据
//           let mut props:Vec<ReportPropertyData> = Vec::new();
//           if  r.props != None{
//             let rprops = r.props.unwrap();
//             for v2 in rprops {
//               props.push(ReportPropertyData{
//                 name: String::from_utf8(v2.name).unwrap(),
//                 value: String::from_utf8(v2.value).unwrap(),
//               });
//             }
//           }

//           // 构建最终数据
//           let rh = ReportDataH{
//             hash: change_set.block.to_string(),
//             value: ReportData{
//               // 报告id
//               id: String::from_utf8(r.id).unwrap(),
//               // 存证企业
//               com: Some(r.com.to_string()),
//               // 操作人员
//               operator: Some(r.operator.to_string()),
//               // 属性
//               props: props,
//             }
//           };
//           let hash_key = "0x".to_string()+&hex::encode(&k.0);
//           report_map.get_mut(&hash_key).unwrap().history.push(rh);
//         }
//       }
//     }
//     let mut reports: Vec<ReportDetail> = Vec::new();
//     for key in hashs {
//       let mut data = report_map.get(&key).unwrap().clone();
//       data.curent_value = Some(data.history[data.history.len()-1].clone());
//       reports.push(data);
//     }

//     Ok(reports)
//   }

//   // 报告详情
//   pub async fn report_detail_hash(&self, hash: &str) -> Result<ReportDetail, Box<dyn Error>>{
//     // let signer = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
//     // let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).
//       skip_type_sizes_check().
//       build().await?;

//     // 获取key
//     let hash_real = hex::decode(str::replace(hash, "0x", "")).unwrap();
//     return self._report_detail(StorageKey(hash_real), client).await
//   }

//   // 报告详情
//   pub async fn report_detail(&self, id: &str) -> Result<ReportDetail, Box<dyn Error>>{
//     // 创建连接
//     let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).
//     skip_type_sizes_check().
//     build().await?;

//     // 获取key
//     let metadata = client.metadata();
//     let key = metadata.module("PacsDeposit")?.storage("Reports")?.map()?.key(&id.as_bytes().to_vec(),);
//     return self._report_detail(key, client).await
//   }

//   async fn _report_detail(&self, hash_real: StorageKey, client: subxt::Client::<DefaultNodeRuntime>) -> Result<ReportDetail, Box<dyn Error>>{
//     let mut keys: Vec<StorageKey> = Vec::new();
//     keys.push(hash_real.clone());
//     // 查询数据
//     // let data = client.fetch_unhashed::<
//     //   Report<
//     //     <DefaultNodeRuntime as System>::AccountId,
//     //     <DefaultNodeRuntime as Timestamp>::Moment
//     //   >
//     // >(StorageKey(hash_real.clone()),None).await.unwrap();
//     // 查询数据
//     let block_hash = client.block_hash(Some(1.into())).await?;
//     let datas = client.query_storage(keys.clone(),block_hash.unwrap(),None).await.unwrap();

//     // 构建返回数据
//     let mut report = ReportDetail{
//       key: "0x".to_string()+&hex::encode(hash_real.clone().0),
//       curent_value: None,
//       history: Vec::new(),
//     };

//     // 构建查询数据
//     for change_set in datas {
//       for (_k, v) in change_set.changes {
//         if v != None {
//           let vdata = v.unwrap();
//           // 解码数据
//           let r: Report<
//             <DefaultNodeRuntime as System>::AccountId,
//             <DefaultNodeRuntime as Timestamp>::Moment
//           > = Decode::decode(&mut &vdata.0[..]).unwrap();

//           // 转换报告属性数据
//           let mut props:Vec<ReportPropertyData> = Vec::new();
//           if  r.props != None{
//             let rprops = r.props.unwrap();
//             for v2 in rprops {
//               props.push(ReportPropertyData{
//                 name: String::from_utf8(v2.name).unwrap(),
//                 value: String::from_utf8(v2.value).unwrap(),
//               });
//             }
//           }

//           // 构建最终数据
//           let rh = ReportDataH{
//             hash: change_set.block.to_string(),
//             value: ReportData{
//               // 报告id
//               id: String::from_utf8(r.id).unwrap(),
//               // 存证企业
//               com: Some(r.com.to_string()),
//               // 操作人员
//               operator: Some(r.operator.to_string()),
//               // 属性
//               props: props,
//             }
//           };
//           report.history.push(rh);
//         }
//       }
//     }
//     report.curent_value = Some(report.history[report.history.len()-1].clone());
//     Ok(report)
//   }
// }
