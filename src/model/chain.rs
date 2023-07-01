use sp_runtime::AccountId32;
use substrate_api_client::{rpc::JsonrpseeClient};
use substrate_api_client::{
    Api, ExtrinsicSigner, PlainTipExtrinsicParams,
};
use sp_core::sr25519;
use tokio::sync::oneshot;
use wetee_runtime::{Signature, Runtime, RuntimeCall};


// 用于存储客户端的连接
pub type ChainApi = Api<
    ExtrinsicSigner<sr25519::Pair, Signature, Runtime>,
    JsonrpseeClient,
    PlainTipExtrinsicParams<Runtime>,
    Runtime,
>;

#[derive(Debug,Clone)]
pub enum QueryKey{
    U64Key(u64),
    U32Key(u32),
    StrKey(String),
    AccountId(AccountId32),
}

#[derive(Debug)]
pub enum Command {
    QueryBlockNumber {
        resp: Responder<u64>,
    },
    QueryValue {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        resp: Responder<Option<Vec<u8>>>,
    },
    QueryMap {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        key: QueryKey,
        resp: Responder<Option<Vec<u8>>>,
    },
    QueryMapAll {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        resp: Responder<Vec<(String, Vec<u8>)>>,
    },
    QueryDoubleMap {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        first: QueryKey,
        second: QueryKey,
        resp: Responder<Option<Vec<u8>>>,
    },
    QueryDoubleMapFirst {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        first: QueryKey,
        resp: Responder<Vec<(String, Vec<u8>)>>,
    },
    SubmitExtrinsic{
        call: RuntimeCall,
        signer: String,
        resp: Responder<String>,
    },
    Close,
}
pub type Responder<T> = oneshot::Sender<anyhow::Result<T>>;