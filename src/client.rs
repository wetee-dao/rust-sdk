use std::sync::Mutex;

use anyhow::Ok;
use codec::Decode;
use once_cell::sync::Lazy;
use sp_core::sr25519;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use substrate_api_client::{GetHeader, rpc::JsonrpseeClient};
use substrate_api_client::{
    Api, ExtrinsicSigner, PlainTipExtrinsicParams, GetStorage,
};
use tokio::sync::oneshot;
use wetee_runtime::{Signature, Runtime, RuntimeCall};


type Responder<T> = oneshot::Sender<anyhow::Result<T>>;

#[derive(Debug,Clone)]
struct StrKey{
    // 数字类型的key
    pub int_key: Option<u64>,
    // 字符串类型的key
    pub str_key: Option<String>,
    // u8 -> 1, u16 -> 2, u32 -> 3, u64 -> 4, u128 -> 5
    // str -> 6
    // acountid -> 7
    pub key_type: u8,
}

#[derive(Debug)]
pub enum Command {
    QueryValue {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        resp: Responder<Vec<u8>>,
    },
    QueryMap {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        key: Vec<u8>,
        resp: Responder<Vec<u8>>,
    },
    QueryDoubleMap {
        req: String,
        value: Vec<u8>,
        resp: Responder<Vec<u8>>,
    },
    SubmitExtrinsic{
        call: RuntimeCall,
        value: Vec<u8>,
        resp: Responder<Vec<u8>>,
    },
    Close,
}

type ChainApi = Api<
    ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
    JsonrpseeClient,
    PlainTipExtrinsicParams<Runtime>,
    Runtime,
>;
pub fn get_block_number(api: &ChainApi) -> Result<(u64, String), anyhow::Error> {
    let header_hash = api.get_finalized_head().unwrap().unwrap();
    let h = api.get_header(Some(header_hash)).unwrap().unwrap();

    Ok((h.number, header_hash.to_string()))
}

/// 区块链连接
#[derive(Debug)]
pub struct Client {
    // 客户端index
    pub index: usize,
    // 任务处理句柄
    pub recver: Option<Receiver<Command>>,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let (tx, rx) = channel::<Command>(10);
        let i = init_worker_send(uri,tx)?;
        Ok(Client { recver:Some(rx),index:i })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client {
            index: index as usize,
            recver: None,
        })
    }

    pub async fn start(&mut self) {
        let url = self.get_url();
        let client = JsonrpseeClient::new(url.as_str()).unwrap();
        let mut api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            JsonrpseeClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();
    
        let recver = self.recver.as_mut().unwrap();
        while let Some(data) = recver.recv().await {
            // let h = get_block_number(&api).unwrap();
            // let _ = data.resp.send(Ok(h.0.to_string()));
            match data {
                Command::QueryValue { storage_prefix, storage_key_name, resp } => {
                    let storagekey = api.metadata().storage_value_key(storage_prefix, storage_key_name).unwrap();
                    let s = api.get_opaque_storage_by_key_hash(storagekey, None).unwrap().unwrap();
                    let _ = resp.send(Ok(s));
                },
                Command::QueryMap { storage_prefix, storage_key_name, resp, key } => {
                    let storagekey = api.metadata().storage_map_key(storage_prefix, storage_key_name).unwrap();
                    let s = api.get_opaque_storage_by_key_hash(storagekey, None).unwrap().unwrap();
                    let _ = resp.send(Ok(s));
                },
                Command::QueryDoubleMap { req, value, resp } => todo!(),
                Command::SubmitExtrinsic { value, resp, call } => todo!(),
                Command::Close => recver.close(),
            }
        }
    }

    // pub async fn call(& self) {
    //     let sender = self.get_sender();
    //     let (resp_tx, resp_rx) = oneshot::channel();
    //     let cmd = Command {
    //       req: "foo".to_string(),
    //       value: "bar".into(),
    //       resp: resp_tx,
    //     };
    //     sender.send(cmd).await.unwrap();
      
    //     let resp = resp_rx.await.unwrap();
    //     println!("GOT = {:?}", resp);
    // }

    pub async fn get_storage_value<V: Decode>(
        & self,
        storage_prefix: &'static str,
        storage_key_name: &'static str
    ) -> anyhow::Result<Option<V>> {
        let sender = self.get_sender();
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::QueryValue {
          storage_prefix,
          storage_key_name,
          resp: resp_tx,
        };
        sender.send(cmd).await.unwrap();
      
        let s = resp_rx.await.unwrap().unwrap();
        Ok(Some(Decode::decode(&mut s.as_slice())?))
    }

    pub fn get_sender(& self) -> Sender<Command> {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        _api_box.get(index).unwrap().1.clone()
    }

    pub fn get_url(& self) -> String {
        let index = self.index;
        let _api_box = WORKER_POOL.try_lock().unwrap();
        _api_box.get(index).unwrap().0.clone()
    }
}


// 全局区块链连接
pub static WORKER_POOL: Lazy<
    Mutex<Vec<(String,Sender<Command>)>,>,
> = Lazy::new(|| Mutex::new(vec![]));

// 获取区块链连接
pub fn init_worker_send(url:String,sender:Sender<Command>) -> anyhow::Result<usize, anyhow::Error> {
    // 连接区块链
    let mut _api_box = WORKER_POOL.lock().unwrap();

    let binding = url.clone();
    _api_box.push((binding,sender));
    Ok(_api_box.len() - 1)
}