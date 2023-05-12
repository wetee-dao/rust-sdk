use sp_runtime::AccountId32;
use substrate_api_client::{GetHeader, rpc::JsonrpseeClient};
use substrate_api_client::{
    Api, ExtrinsicSigner, PlainTipExtrinsicParams,
};
use sp_core::sr25519;
use tokio::sync::oneshot;
use wetee_runtime::{Signature, Runtime, RuntimeCall};


// 用于存储客户端的连接
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

#[derive(Debug,Clone)]
pub enum QueryKey{
    IntKey(u64),
    StrKey(String),
    AccountId(AccountId32),
}

#[derive(Debug)]
pub enum Command {
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
    QueryDoubleMap {
		storage_prefix: &'static str,
		storage_key_name: &'static str,
        first: QueryKey,
        second: QueryKey,
        resp: Responder<Option<Vec<u8>>>,
    },
    SubmitExtrinsic{
        call: RuntimeCall,
        signer: String,
        resp: Responder<String>,
    },
    Close,
}
pub type Responder<T> = oneshot::Sender<anyhow::Result<T>>;