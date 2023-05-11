#![allow(unused_imports)]
#![cfg(test)]

use once_cell::sync::OnceCell;
// use crate::hander::{balance::Balance, wetee_app::Wetee};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::print;
use std::sync::Mutex;
use std::{thread, time::Duration};

use crate::hander::balance::Balance;
use crate::hander::wetee_app::Wetee;
use crate::hander::wetee_asset::WeteeAsset;
use crate::hander::wetee_dao::WeteeDAO;
use crate::hander::wetee_gov::WeteeGov;
use crate::hander::wetee_guild::WeteeGuild;
use crate::hander::wetee_project::WeteeProject;
use crate::model::dao::WithGov;

use super::*;
const SEED: &str = "gloom album notable jewel divorce never trouble lesson month neck sign harbor";
const URL: &str = "ws://127.0.0.1:3994";
pub static DAO_ID: OnceCell<u64> = OnceCell::new();

// #[tokio::test]
// async fn test_sign() {
//     let key =
//         account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();

//     let (address, _ss58address) = account::add_keyring(key, "123456".to_owned()).unwrap();
//     let str = account::sign_from_address(address, String::from("test")).unwrap();

//     println!("str {:?}", str);
// }

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
async fn test_blance() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client: Client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut balance = Balance::new(client.clone());
    balance.balance(address.clone()).unwrap();

    let util: u128 = 1000000000000;
    balance
        .init_from_pair(address.clone(), 1000 * util)
        .unwrap();

    balance
        .init_from_pair(
            "0x7e5221ab36e1d1214b24a2b1975603fe566c94828571775a49d8ca94c773f513".to_string(),
            1000 * util,
        )
        .unwrap();
}

#[tokio::test]
async fn test_dao() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut dao = WeteeDAO::new(client);
    let dao_id = dao.next_dao_id().unwrap();
    println!("dao ===> {}", dao_id);
    DAO_ID.set(dao_id).unwrap();

    dao.create_dao(
        address.clone(),
        "WeteeDAO".to_string(),
        "For the freedom of programming".to_string(),
        "{}".to_string(),
    )
    .unwrap();
}

#[tokio::test]
async fn test_dao_asset() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut dao = WeteeAsset::new(client);
    let dao_id = DAO_ID.get().unwrap();
    dao.create_asset(
        address.clone(),
        dao_id.clone(),
        "TET".to_string(),
        "test".to_string(),
        10000,
        10000,
    )
    .unwrap();

    let asset_b = dao.balance(*dao_id, address).unwrap();
    println!("asset_b => {:?}", asset_b);
}

#[tokio::test]
async fn test_dao_guild() {
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();

    let mut dao = WeteeGuild::new(client);
    let dao_id = DAO_ID.get().unwrap();
    println!("dao_id => {:?}", dao_id);
    let gs = dao.guild_list(*dao_id).unwrap();

    println!("gs => {:?}", gs);

    let g = dao.guild_info(*dao_id, 0).unwrap();

    println!("g => {:?}", g);
}

#[tokio::test]
async fn test_dao_roadmap() {
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();

    let mut dao = WeteeDAO::new(client);
    let dao_id = DAO_ID.get().unwrap();
    let rs = dao.roadmap_list(*dao_id, 2023).unwrap();

    println!("rs => {:?}", rs);
}

#[tokio::test]
async fn test_dao_projects() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    println!("address {:?}", ss58address);

    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();

    let mut dao = WeteeProject::new(client.clone());
    let dao_id = DAO_ID.get().unwrap();
    let ps = dao.project_list(*dao_id).unwrap();
    println!("项目列表 => {:?}", ps);

    dao.create_project(
        address.clone(),
        dao_id.clone(),
        "test".to_string(),
        "test".to_string(),
        Some(WithGov {
            run_type: 1,
            amount: 10,
            member: wetee_gov::MemmberData::GLOBAL,
        }),
    )
    .unwrap();

    let mut gov = WeteeGov::new(client.clone());

    gov.set_runment_period(
        address.clone(),
        dao_id.clone(),
        1,
        Some(WithGov {
            run_type: 2,
            amount: 0,
            member: wetee_gov::MemmberData::GLOBAL,
        }),
    )
    .unwrap();

    gov.set_voting_period(
        address.clone(),
        dao_id.clone(),
        1,
        Some(WithGov {
            run_type: 2,
            amount: 0,
            member: wetee_gov::MemmberData::GLOBAL,
        }),
    )
    .unwrap();

    let props = gov.pending_referendum_list(dao_id.clone()).unwrap();
    println!("待开始投票 => {:?}", props);

    gov.referendum_list(5000).unwrap();

    gov.votes_of_user(address.clone(), dao_id.clone()).unwrap();
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
