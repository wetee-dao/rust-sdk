#![allow(unused_imports)]
#![cfg(test)]

// use crate::hander::{balance::Balance, wetee_app::Wetee};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::print;
use std::{thread, time::Duration};

use crate::hander::balance::Balance;
use crate::hander::wetee_app::Wetee;

use super::*;
const SEED: &str = "gloom album notable jewel divorce never trouble lesson month neck sign harbor";
const URL: &str = "ws://127.0.0.1:9944";

#[test]
pub fn test_seed() {
    let seed_str = account::generate();
    let seeds: Vec<&str> = seed_str.split(' ').collect();
    println!("seed_str => {:?}", seed_str);
    println!("seeds => {:?}", seeds);
}

#[test]
fn test_add_seed_keyring() {
    let (address, _) = account::add_keyring_from_seed(SEED.into()).unwrap();
    let pair = account::get_from_address(address.clone()).unwrap();

    let address2 = account::format_public_key::<Pair>(pair.public().into());
    println!("address => {:?} ||| address2 => {:?}", address, address2);

    assert_eq!(address, address2)
}

// #[test]
// fn test_add_keyring() {
//     let key =
//         account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();
//     let jstr = serde_json::to_string(&key).unwrap();
//     println!("jstr => {:?}", jstr);

//     assert!(account::add_keyring(key.clone(), "1234567".to_owned()).is_err());

//     let (address, _ss58address) = account::add_keyring(key, "123456".to_owned()).unwrap();
//     let pair = account::get_from_address(address.clone()).unwrap();

//     let address2 = account::format_public_key::<Pair>(pair.public().into());
//     println!("address => {:?} ||| address2 => {:?}", address, address2);

//     assert_eq!(address, address2)
// }

#[tokio::test]
async fn test_client() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let mut client: Client = Client::new(URL.to_string()).unwrap();

    println!("address {:?}", ss58address);

    let (block_number, _) = client.get_block_number().await.unwrap();
    assert!(block_number > 0);

    println!("block_number {:?}", block_number);

    let mut balance = Balance::new(client);
    let (free, _, _, _) = balance.amount(ss58address.clone()).await.unwrap();
}

// #[tokio::test]
// async fn test_wetee() {
//     let (_, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     let pool = wetee.get_wait_pool().await.unwrap();
//     println!("poolpool ===> {:?}", pool);
// }

// #[tokio::test]
// async fn test_wetee_get_app() {
//     let (_, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     let pool = wetee.get_app(1).await.unwrap();
//     println!("poolpool ===> {:?}", pool);
// }

// #[tokio::test]
// async fn test_wetee_run_app() {
//     let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     // let pool = wetee.get_app(1).await.unwrap();

//     wetee.run_app(address, 1).await.unwrap();
//     // println!("poolpool ===> {:?}", pool);
// }

#[tokio::test]
async fn test_sign() {
    let key =
        account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();

    let (address, _ss58address) = account::add_keyring(key, "123456".to_owned()).unwrap();
    let str = account::sign_from_address(address, String::from("test")).unwrap();

    println!("str {:?}", str);
}
