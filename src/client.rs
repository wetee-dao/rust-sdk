use crate::{
    account,
    model::chain::{Command, QueryKey},
};

use codec::Decode;
use once_cell::sync::Lazy;
use sp_core::sr25519;
use std::sync::Mutex;
use substrate_api_client::{
    rpc::JsonrpseeClient, Api, ExtrinsicSigner, GetHeader, GetStorage, PlainTipExtrinsicParams,
    SubmitAndWatchUntilSuccess,
};
use tokio::sync::{
    mpsc::{channel, Sender},
    oneshot,
};
use wetee_runtime::{Runtime, RuntimeCall, Signature};

/// 区块链连接
#[derive(Debug)]
pub struct Client {
    // 客户端index
    pub index: usize,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = init_worker_send(uri)?;
        Ok(Client { index: i })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client {
            index: index as usize,
        })
    }

    pub fn get_status(&self) -> anyhow::Result<u8, anyhow::Error> {
        let _status_box = WORKER_STATUS.try_lock().unwrap();
        if self.index >= _status_box.len() {
            return Err(anyhow::anyhow!("client not start"));
        }
        let status = _status_box.get(self.index).unwrap();
        Ok(status.0.clone())
    }

    pub async fn stop(&self) -> anyhow::Result<(), anyhow::Error> {
        let sender = self.get_sender()?;
        let cmd = Command::Close;
        sender.send(cmd).await.unwrap();

        let mut _status_box = WORKER_STATUS.try_lock().unwrap();
        if self.index >= _status_box.len() {
            return Err(anyhow::anyhow!("client not start"));
        }

        let status = _status_box.get_mut(self.index).unwrap();
        status.0 = 3;

        self.set_sender(None)?;

        Ok(())
    }

    pub async fn get_block_number(&self) -> anyhow::Result<u64, anyhow::Error> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryBlockNumber { resp: resp_tx };
        sender.send(cmd).await.unwrap();

        let s = resp_rx.await.unwrap().unwrap();
        Ok(s)
    }

    pub async fn start(&mut self) -> anyhow::Result<bool, anyhow::Error> {
        let url = self.get_url();
        let client = JsonrpseeClient::new(url.as_str()).unwrap();
        let mut api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            JsonrpseeClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();
        let (tx, mut rx) = channel::<Command>(50);
        self.set_sender(Some(tx))?;

        while let Some(data) = rx.recv().await {
            match data {
                Command::QueryBlockNumber { resp } => {
                    let header_hash = api.get_finalized_head().unwrap().unwrap();
                    let h = api.get_header(Some(header_hash)).unwrap().unwrap();

                    let _ = resp.send(Ok(h.number));
                }
                Command::QueryValue {
                    storage_prefix,
                    storage_key_name,
                    resp,
                } => {
                    let storagekey = api
                        .metadata()
                        .storage_value_key(storage_prefix, storage_key_name)
                        .unwrap();
                    let s = api
                        .get_opaque_storage_by_key_hash(storagekey, None)
                        .unwrap();
                    let _ = resp.send(Ok(s));
                }
                Command::QueryMap {
                    storage_prefix,
                    storage_key_name,
                    resp,
                    key,
                } => {
                    let storagekey = match key {
                        QueryKey::IntKey(v) => api
                            .metadata()
                            .storage_map_key(storage_prefix, storage_key_name, v)
                            .unwrap(),
                        QueryKey::StrKey(v) => api
                            .metadata()
                            .storage_map_key(storage_prefix, storage_key_name, v)
                            .unwrap(),
                        QueryKey::AccountId(v) => api
                            .metadata()
                            .storage_map_key(storage_prefix, storage_key_name, v)
                            .unwrap(),
                    };
                    let s = api
                        .get_opaque_storage_by_key_hash(storagekey, None)
                        .unwrap();
                    let _ = resp.send(Ok(s));
                }
                Command::QueryDoubleMap {
                    storage_prefix,
                    storage_key_name,
                    first,
                    second,
                    resp,
                } => {
                    let storagekey = match first {
                        QueryKey::IntKey(v) => match second {
                            QueryKey::IntKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::StrKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::AccountId(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                        },
                        QueryKey::StrKey(v) => match second {
                            QueryKey::IntKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::StrKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::AccountId(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                        },
                        QueryKey::AccountId(v) => match second {
                            QueryKey::IntKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::StrKey(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                            QueryKey::AccountId(v2) => api
                                .metadata()
                                .storage_double_map_key(storage_prefix, storage_key_name, v, v2)
                                .unwrap(),
                        },
                    };
                    let s = api
                        .get_opaque_storage_by_key_hash(storagekey, None)
                        .unwrap();
                    let _ = resp.send(Ok(s));
                }
                Command::QueryDoubleMapFirst {
                    storage_prefix,
                    storage_key_name,
                    first,
                    resp,
                } => {
                    let storagekey = match first {
                        QueryKey::IntKey(v) => api
                            .get_storage_double_map_key_prefix(storage_prefix, storage_key_name, v)
                            .unwrap(),
                        QueryKey::StrKey(v) => api
                            .get_storage_double_map_key_prefix(storage_prefix, storage_key_name, v)
                            .unwrap(),
                        QueryKey::AccountId(v) => api
                            .get_storage_double_map_key_prefix(storage_prefix, storage_key_name, v)
                            .unwrap(),
                    };
                    let storage_keys = api
                        .get_storage_keys_paged(Some(storagekey), 1000, None, None)
                        .unwrap();

                    let mut results = vec![];
                    for storage_key in storage_keys.iter() {
                        let storage_data: Option<Vec<u8>> = api
                            .get_opaque_storage_by_key_hash(storage_key.clone(), None)
                            .unwrap();
                        let hash = "0x".to_owned() + &hex::encode(storage_key.clone().0);
                        match storage_data {
                            Some(storage) => results.push((hash, storage)),
                            None => {}
                        }
                    }

                    let _ = resp.send(Ok(results));
                }
                Command::SubmitExtrinsic { resp, call, signer } => {
                    let from_pair = account::get_from_address(signer.clone()).unwrap();
                    api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));
                    let signer_nonce = api.get_nonce().unwrap();
                    let xt = api.compose_extrinsic_offline(call, signer_nonce);
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
                }
                Command::Close => rx.close(),
            }
        }

        Ok(true)
    }

    pub async fn get_storage_value<V: Decode>(
        &self,
        storage_prefix: &'static str,
        storage_key_name: &'static str,
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
        &self,
        storage_prefix: &'static str,
        storage_key_name: &'static str,
        key: QueryKey,
    ) -> anyhow::Result<Option<V>> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryMap {
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
        &self,
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

    pub async fn get_storage_double_map_first<V: Decode>(
        &self,
        storage_prefix: &'static str,
        storage_key_name: &'static str,
        first: QueryKey,
    ) -> anyhow::Result<Vec<(String, V)>> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryDoubleMapFirst {
            storage_prefix,
            storage_key_name,
            first,
            resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();

        let s = resp_rx.await.unwrap().unwrap();
        let mut results = vec![];
        for storage in s.iter() {
            results.push((
                storage.0.clone(),
                Decode::decode(&mut storage.1.as_slice())?,
            ));
        }
        Ok(results)
    }

    pub async fn send_and_sign(&self, call: RuntimeCall, signer: String) -> anyhow::Result<()> {
        let sender = self.get_sender()?;
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::SubmitExtrinsic {
            call,
            signer,
            resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();

        let _ = resp_rx.await.unwrap().unwrap();
        Ok(())
    }

    pub fn get_sender(&self) -> anyhow::Result<Sender<Command>> {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        let _status_box = WORKER_STATUS.try_lock().unwrap();
        if index >= _status_box.len() || index >= _api_box.len() {
            return Err(anyhow::anyhow!("client not start"));
        }
        let status = _status_box.get(index).unwrap();
        let sender = _api_box.get(index).unwrap();

        if status.0 == 3 {
            return Err(anyhow::anyhow!("client is stop"));
        }

        match &sender.1 {
            Some(s) => Ok(s.clone()),
            None => Err(anyhow::anyhow!("client not start")),
        }
    }

    pub fn set_sender(&self, s: Option<Sender<Command>>) -> anyhow::Result<bool> {
        let index = self.index;
        let mut _api_box = WORKER_POOL.try_lock().unwrap();
        if index >= _api_box.len() {
            return Err(anyhow::anyhow!("client not start"));
        }
        let sender = _api_box.get_mut(index).unwrap();

        sender.1 = s;
        Ok(true)
    }

    pub fn get_url(&self) -> String {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        _api_box.get(index).unwrap().0.clone()
    }
}

// 全局区块链连接
pub static WORKER_POOL: Lazy<Mutex<Vec<(String, Option<Sender<Command>>)>>> =
    Lazy::new(|| Mutex::new(vec![]));
// 全局区块链状态
pub static WORKER_STATUS: Lazy<Mutex<Vec<(u8, usize)>>> = Lazy::new(|| Mutex::new(vec![]));

// 获取区块链连接
pub fn init_worker_send(url: String) -> anyhow::Result<usize, anyhow::Error> {
    // 连接区块链
    let mut _api_box = WORKER_POOL.lock().unwrap();
    let mut _api_status = WORKER_STATUS.lock().unwrap();

    let curl: String = url.clone();
    _api_box.push((curl, None));
    _api_status.push((0, _api_box.len() - 1));

    Ok(_api_box.len() - 1)
}
