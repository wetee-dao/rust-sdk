use crate::{model::chain::{QueryKey, Command}, account};

use std::sync::Mutex;
use codec::Decode;
use once_cell::sync::Lazy;
use sp_core::{sr25519};
use substrate_api_client::{rpc::JsonrpseeClient, Api, ExtrinsicSigner, PlainTipExtrinsicParams, GetStorage, SubmitAndWatchUntilSuccess};
use tokio::sync::{mpsc::{channel, Sender}, oneshot};
use wetee_runtime::{Signature, Runtime,RuntimeCall};

/// 区块链连接
#[derive(Debug)]
pub struct Client {
    // 客户端index
    pub index: usize,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = init_worker_send(uri)?;
        Ok(Client { index:i })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client { index: index as usize })
    }

    pub async fn start(&mut self) -> anyhow::Result<bool, anyhow::Error> {
        let url = self.get_url();
        let client = JsonrpseeClient::new(url.as_str()).unwrap();
        let mut api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            JsonrpseeClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client).unwrap();
        let (tx, mut rx) = channel::<Command>(50);
        self.set_sender(tx)?;

        while let Some(data) = rx.recv().await {
            match data {
                Command::QueryValue { storage_prefix, storage_key_name, resp } => {
                    let storagekey = api.metadata().storage_value_key(storage_prefix, storage_key_name).unwrap();
                    let s = api.get_opaque_storage_by_key_hash(storagekey, None).unwrap();
                    let _ = resp.send(Ok(s));
                },
                Command::QueryMap { storage_prefix, storage_key_name, resp, key } => {
                    let storagekey = match key {
                        QueryKey::IntKey(v) => api.metadata().storage_map_key(storage_prefix, storage_key_name,v).unwrap(),
                        QueryKey::StrKey(v) => api.metadata().storage_map_key(storage_prefix, storage_key_name,v).unwrap(),
                        QueryKey::AccountId(v) => api.metadata().storage_map_key(storage_prefix, storage_key_name,v).unwrap(),
                    };
                    let s = api.get_opaque_storage_by_key_hash(storagekey, None).unwrap();
                    let _ = resp.send(Ok(s));
                },
                Command::QueryDoubleMap { storage_prefix, storage_key_name, first, second, resp  } =>{
                    let storagekey = match first {
                        QueryKey::IntKey(v) => match second {
                            QueryKey::IntKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::StrKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::AccountId(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                        },
                        QueryKey::StrKey(v) => match second {
                            QueryKey::IntKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::StrKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::AccountId(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                        },
                        QueryKey::AccountId(v) => match second {
                            QueryKey::IntKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::StrKey(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                            QueryKey::AccountId(v2) => api.metadata().storage_double_map_key(storage_prefix, storage_key_name,v,v2).unwrap(),
                        },
                    };
                    let s = api.get_opaque_storage_by_key_hash(storagekey, None).unwrap();
                    let _ = resp.send(Ok(s));
                },
                Command::SubmitExtrinsic { resp, call, signer } => {
                    let signer_nonce = api.get_nonce().unwrap();
                    let xt = api.compose_extrinsic_offline(call, signer_nonce);
                    let from_pair = account::get_from_address(signer.clone()).unwrap();
                    api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));
                    // 发送请求
                    let result = api.submit_and_watch_extrinsic_until_success(xt, false);
                    match result {
                        Ok(report) => {
                            println!(
                                "[+] Extrinsic got included in block {:?}",
                                report.block_hash
                            );
                            let _ = resp.send(Ok(report.block_hash.unwrap().to_string()));
                        }
                        Err(e) => {
                            println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                            let string_error = format!("{:?}", e);
                            let _ = resp.send(Err(anyhow::anyhow!(string_error)));
                        }
                    };
                },
                Command::Close => rx.close(),
            }
        }
    
        Ok(true)
    }

    pub async fn get_storage_value<V: Decode>(
        & self,
        storage_prefix: &'static str,
        storage_key_name: &'static str
    ) -> anyhow::Result<Option<V>> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryValue {
          storage_prefix,
          storage_key_name,
          resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();
      
        let s = resp_rx.await.unwrap().unwrap();
        match s {
			Some(storage) => Ok(Some(Decode::decode(&mut storage.as_slice())?)),
			None => Ok(None),
		}
    }

    pub async fn get_storage_map<V: Decode>(
        & self,
        storage_prefix: &'static str,
        storage_key_name: &'static str,
        key: QueryKey,
    ) -> anyhow::Result<Option<V>> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryMap{
          storage_prefix,
          storage_key_name,
          key,
          resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();
      
        let s = resp_rx.await.unwrap().unwrap();
        match s {
			Some(storage) => Ok(Some(Decode::decode(&mut storage.as_slice())?)),
			None => Ok(None),
		}
    }

    pub async fn get_storage_double_map<V: Decode>(
        & self,
        storage_prefix: &'static str,
        storage_key_name: &'static str,
        first: QueryKey,
        second: QueryKey,
    ) -> anyhow::Result<Option<V>> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryDoubleMap {
          storage_prefix,
          storage_key_name,
          first,
          second,
          resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();
      
        let s = resp_rx.await.unwrap().unwrap();
        match s {
			Some(storage) => Ok(Some(Decode::decode(&mut storage.as_slice())?)),
			None => Ok(None),
		}
    }

    pub async fn send_and_sign(
        & self,
        call: RuntimeCall,
        signer: String
    ) -> anyhow::Result<()> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SubmitExtrinsic { call, signer, resp: resp_tx };
        sender.send(cmd).await.unwrap();
      
        let _ = resp_rx.await.unwrap().unwrap();
        Ok(())
    }

    pub fn get_sender(& self) -> anyhow::Result<Sender<Command>> {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        let sender = _api_box.get(index).unwrap();

        match &sender.1 {
            Some(s) => Ok(s.clone()),
            None => Err(anyhow::anyhow!("client not start")),
        }
    }

    pub fn set_sender(& self,s: Sender<Command>) -> anyhow::Result<bool> {
        let index = self.index;
        let mut _api_box = WORKER_POOL.try_lock().unwrap();
        let sender = _api_box.get_mut(index).unwrap();

        sender.1 = Some(s);
        Ok(true)
    }

    pub fn get_url(& self) -> String {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        _api_box.get(index).unwrap().0.clone()
    }
}


// 全局区块链连接
pub static WORKER_POOL: Lazy<
    Mutex<Vec<(String,Option<Sender<Command>>)>,>,
> = Lazy::new(|| Mutex::new(vec![]));

// 获取区块链连接
pub fn init_worker_send(url:String) -> anyhow::Result<usize, anyhow::Error> {
    // 连接区块链
    let mut _api_box = WORKER_POOL.lock().unwrap();

    let curl: String = url.clone();
    _api_box.push((curl,None));

    Ok(_api_box.len() - 1)
}