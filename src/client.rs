use std::sync::Mutex;

use anyhow::Ok;
use once_cell::sync::Lazy;
use sp_core::sr25519;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use substrate_api_client::{GetHeader, rpc::JsonrpseeClient};
use substrate_api_client::{
    Api, ExtrinsicSigner, PlainTipExtrinsicParams,
};
use wetee_runtime::{Signature, Runtime};
use crate::chain::*;

/// 区块链连接
#[derive(Debug, Clone)]
pub struct Client {
    // u32
    pub index: usize,
    // 链接
    // api: Option<String>,
}

impl Client {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        let i = get_api(uri.clone())?;
        Ok(Client { index: i })
    }

    pub fn from_index(index: u32) -> anyhow::Result<Self, anyhow::Error> {
        Ok(Client {
            index: index as usize,
        })
    }

    pub fn get_block_number(&mut self) -> Result<(u64, String), anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.index).unwrap();

        let header_hash = api.get_finalized_head().unwrap().unwrap();
        let h = api.get_header(Some(header_hash)).unwrap().unwrap();

        Ok((h.number, header_hash.to_string()))
    }
}


// 全局区块链连接
pub static WORKER_POOL: Lazy<
    Mutex<Vec<(Sender<String>,Receiver<String>)>,>,
> = Lazy::new(|| Mutex::new(vec![]));

/// 区块链连接
#[derive(Debug)]
pub struct Worker {
    // u32
    // pub index: usize,
    pub sender: Sender<String>,
    pub recver: Receiver<String>,
}

impl Worker {
    pub fn new(uri: String) -> anyhow::Result<Self, anyhow::Error> {
        // let i = get_worker(uri.clone())?;
        let (tx, rx) = channel::<String>(10);
        Ok(Worker { sender:tx,recver:rx })
    }

    pub async fn start(&mut self) {
        // 连接区块链
        // let mut _api_box = WORKER_POOL.lock().unwrap();
        // let (tx,rx) = _api_box.get_mut(self.index).unwrap();
        println!("start client");
    
        // 获取区块链接口
        let client = JsonrpseeClient::new("ws://127.0.0.1:3994").unwrap();
        let api = Api::<
            ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
            JsonrpseeClient,
            PlainTipExtrinsicParams<Runtime>,
            Runtime,
        >::new(client)
        .unwrap();
    
        println!("cliented");
        while let Some(data) = self.recver.recv().await {
            // println!("Received data: {:?}", data);
            let header_hash = api.get_finalized_head().unwrap().unwrap();
            let h = api.get_header(Some(header_hash)).unwrap().unwrap();
            println!("h.number: {:?}", h.number);
        }
    }

    pub async fn send(& self) {
        self.sender.send("Hello world".to_owned()).await.unwrap();
    }
}


// 获取区块链连接
pub fn get_worker(url: String) -> anyhow::Result<usize, anyhow::Error> {
    // 连接区块链
    let mut _api_box = WORKER_POOL.lock().unwrap();
    let (tx, rx) = channel::<String>(10);

    _api_box.push((tx,rx));

    Ok(_api_box.len() - 1)
}