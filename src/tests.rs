#![allow(unused_imports)]
#![cfg(test)]

use sp_runtime::print;
use subxt::ext::sp_core::{sr25519::Pair, Pair as TraitPair};

use crate::hander::balance::Balance;

use super::*;
const SEED: &str = "gloom album notable jewel divorce never trouble lesson month neck sign harbor";

#[test]
pub fn test_seed() {
    let seed_str = account::generate();
    let seeds: Vec<&str> = seed_str.split(' ').collect();
    println!("seed_str => {:?}", seed_str);
    println!("seeds => {:?}", seeds);
}

#[test]
fn test_add_seed_keyring() {
    let address = account::add_keyring_from_seed(SEED.into()).unwrap();
    let pair = account::get_from_address(address.clone()).unwrap();

    let address2 = account::format_public_key::<Pair>(pair.public().into());
    println!("address => {:?} ||| address2 => {:?}", address, address2);

    assert_eq!(address, address2)
}

#[test]
fn test_add_keyring() {
    let key =
        account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();
    let jstr = serde_json::to_string(&key).unwrap();
    println!("jstr => {:?}", jstr);

    assert!(account::add_keyring(key.clone(), "1234567".to_owned()).is_err());

    let address = account::add_keyring(key, "123456".to_owned()).unwrap();
    let pair = account::get_from_address(address.clone()).unwrap();

    let address2 = account::format_public_key::<Pair>(pair.public().into());
    println!("address => {:?} ||| address2 => {:?}", address, address2);

    assert_eq!(address, address2)
}

#[tokio::test]
async fn test_client() {
    let address = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client: Client = Client::new("wss://chain.asyou.me:443".to_string())
        .await
        .unwrap();

    println!("{:?}", address);

    let (block_number, _) = client.get_block_number().await.unwrap();
    assert!(block_number > 0);

    let mut balance = Balance::new(client);
    let (free, _, _, _) = balance.amount(address.clone()).await.unwrap();

    assert!(free == 0);
}
