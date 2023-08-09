use codec::{Decode, Encode};
use sp_core::sr25519;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::AccountId32;
use substrate_api_client::ac_primitives::{Config, AccountData, AssetTipExtrinsicParams, ExtrinsicSigner};
use substrate_api_client::rpc::JsonrpseeClient;
use substrate_api_client::Api;
use tokio::sync::oneshot;
use wetee_runtime::{RuntimeCall, Header, Block, Nonce, BlockNumber, Hash, AccountId, Address, Signature};


/// Default set of commonly used types by Substrate kitchensink runtime.
#[derive(Decode, Encode, Clone, Eq, PartialEq, Debug)]
pub struct WeteeConfig {}
impl Config for WeteeConfig {
	type Index = Nonce;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type AccountId = AccountId;
	type Address = Address;
	type Signature = Signature;
	type Hasher = BlakeTwo256;
	type Header = Header;
	type AccountData = AccountData<Self::Balance>;
	type ExtrinsicParams = AssetTipExtrinsicParams<Self>;
	type CryptoKey = sr25519::Pair;
	type ExtrinsicSigner = ExtrinsicSigner<Self>;
	type Block = Block;
	type Balance = u128;
	type ContractCurrency = u128;
	type StakingBalance = u128;
}

// 用于存储客户端的连接
pub type ChainApi = Api<WeteeConfig, JsonrpseeClient>;

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