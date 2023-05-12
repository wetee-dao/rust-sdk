use crate::{
    chain::*,
    model::{account::*, err::AccountError},
};
use bip39::{Language, Mnemonic, MnemonicType};
use sp_core::{
    crypto::{Ss58AddressFormat, Ss58Codec},
    hexdisplay::{AsBytesRef, HexDisplay},
    sr25519::{Pair, Public},
    Pair as TraitPair,
};
use std::collections::HashMap;
use xsalsa20poly1305::{
    aead::{generic_array::GenericArray, Aead},
    KeyInit, XSalsa20Poly1305,
};

/// 加密噪点
const NONCE: &[u8; 24] = &[
    0x69, 0x69, 0x6e, 0xe9, 0x55, 0xb6, 0x2b, 0x73, 0xcd, 0x62, 0xbd, 0xa8, 0x75, 0xfc, 0x73, 0xd6,
    0x82, 0x19, 0xe0, 0x03, 0x6b, 0x7a, 0x0b, 0x37,
];

/// 生成账户种子
pub fn generate() -> String {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    return mnemonic.phrase().to_string();
}

/// 获取账户信息
pub fn get_seed_phrase(
    seed_str: String,
    name: String,
    password: String,
) -> anyhow::Result<KeyringJSON, AccountError> {
    // 助记词换账户
    let (pair, seed) = Pair::from_phrase(&seed_str, None).unwrap();

    // 获取公钥
    let public_key = pair.public();
    // println!("Secret phrase:  {}", seed_str);
    // println!("Secret   seed:  {}", format_seed::<Pair>(seed));
    // println!(
    //     "Public    key:  {}",
    //     format_public_key::<Pair>(public_key.clone())
    // );
    // println!(
    //     "SS58  Address:  {}",
    //     public_key.to_ss58check_with_version(Ss58AddressFormat::custom(42))
    // );

    // 因key必须32位，即重复key得到32位key
    let pwb = password.as_bytes().to_vec();
    let pwb_len = pwb.len();
    let mut keypw = vec![];
    if pwb_len == 32 {
        keypw = password.as_bytes().to_vec();
    } else {
        for i in 0..32 {
            let index = i % pwb_len;
            keypw.push(pwb[index]);
        }
    }
    let key = GenericArray::from_slice(keypw.as_bytes_ref());
    let nonce = GenericArray::from_slice(NONCE);

    // 获取加密对象
    let cipher = XSalsa20Poly1305::new(key);

    // 加密文本
    let ciphertext = cipher.encrypt(nonce, seed.as_slice()).unwrap();

    // 账户元数据
    let mut meta: HashMap<String, String> = HashMap::new();
    meta.insert("name".to_string(), name);
    meta.insert("ss58_prefix".to_string(), 42.to_string());

    Ok(KeyringJSON {
        address: String::from("0x") + hex::encode(public_key.0).as_str(),
        encoded: hex::encode(ciphertext),
        encoding: KeyringJSONEncoding {
            content: vec!["sr25519".to_string()],
            typex: "xsalsa20-poly1305".to_string(),
            version: "wetee-0".to_string(),
        },
        meta,
    })
}

/// 获取账户 Pair
pub fn pair_from_password(
    keyring: KeyringJSON,
    password: String,
) -> anyhow::Result<Pair, AccountError> {
    let ciphertext = hex::decode(keyring.encoded).unwrap();
    // 因key必须32位，即重复key得到32位key
    let pwb = password.as_bytes().to_vec();
    let pwb_len = pwb.len();
    let mut keypw = vec![];
    if pwb_len == 32 {
        keypw = password.as_bytes().to_vec();
    } else {
        for i in 0..32 {
            let index = i % pwb_len;
            keypw.push(pwb[index]);
        }
    }
    let key = GenericArray::from_slice(keypw.as_bytes_ref());
    let nonce = GenericArray::from_slice(NONCE);

    // 获取加密对象
    let cipher = XSalsa20Poly1305::new(key);

    // 解码
    let seed_result = cipher.decrypt(nonce, ciphertext.as_slice());
    if seed_result.is_err() {
        return Err(AccountError::InvalidPassword(password));
    }

    let seed = seed_result.unwrap();
    let pair = Pair::from_seed_slice(&seed.as_slice()).unwrap();
    // let public_key = pair.public();
    // println!(
    //     "import Public key:  {}",
    //     format_public_key::<Pair>(public_key.clone())
    // );
    Ok(pair)
}

// 获取账户
pub fn get_from_address(address: String) -> anyhow::Result<Pair, AccountError> {
    let mut _key_box = KERINGS.lock().unwrap();
    let pair = _key_box.get(&address).unwrap();
    Ok(pair.clone())
}

// 获取账户
pub fn get_from_ss58(ss58: String) -> anyhow::Result<Pair, AccountError> {
    let public_key = Public::from_ss58check(&ss58).unwrap();
    let address = format_public_key::<Pair>(public_key.clone());
    let mut _key_box = KERINGS.lock().unwrap();
    let pair = _key_box.get(&address).unwrap();
    Ok(pair.clone())
}

// 获取账户
pub fn address_to_ss58(address: String, prefix: u16) -> anyhow::Result<String, AccountError> {
    let public_key = Public::from_string(&address).unwrap();
    Ok(public_key.to_ss58check_with_version(Ss58AddressFormat::custom(prefix)))
}

// 获取账户
pub fn ss58_to_address(address: String) -> anyhow::Result<String, AccountError> {
    let public_key = Public::from_ss58check(&address).unwrap();
    let address = format_public_key::<Pair>(public_key.clone());
    Ok(address)
}

// 添加账户
pub fn add_keyring_from_seed(seed_str: String) -> anyhow::Result<(String, String), AccountError> {
    // 助记词换账户
    let (pair, _seed) = Pair::from_phrase(&seed_str, None).unwrap();

    let public_key = pair.public();
    let address = format_public_key::<Pair>(public_key.clone());

    let mut _key_box = KERINGS.lock().unwrap();
    _key_box.insert(address.clone(), pair);

    let ss58address = public_key.to_ss58check_with_version(Ss58AddressFormat::custom(42));
    Ok((address, ss58address))
}

// 添加密码key
pub fn add_keyring(
    keyring: KeyringJSON,
    password: String,
) -> anyhow::Result<(String, String), AccountError> {
    // 获取账户
    let pair = pair_from_password(keyring, password)?;

    let public_key = pair.public();
    let address = format_public_key::<Pair>(public_key.clone());

    let mut _key_box = KERINGS.lock().unwrap();
    _key_box.insert(address.clone(), pair);

    let ss58address = public_key.to_ss58check_with_version(Ss58AddressFormat::custom(42));

    Ok((address, ss58address))
}

// 添加密码 pair
pub fn add_pair(
    pair: Pair,
) -> anyhow::Result<(String, String), AccountError> {
    let public_key = pair.public();
    let address = format_public_key::<Pair>(public_key.clone());

    let mut _key_box = KERINGS.lock().unwrap();
    _key_box.insert(address.clone(), pair);

    let ss58address = public_key.to_ss58check_with_version(Ss58AddressFormat::custom(42));

    Ok((address, ss58address))
}

// 添加密码key
pub fn sign_from_address(address: String, ctx: String) -> anyhow::Result<String, AccountError> {
    // 获取账户
    let signer = get_from_address(address).expect("Could not obtain stash signer pair");
    let sign = signer.sign(ctx.as_bytes());

    let str = hex::encode(sign.0);

    Ok("0x".to_owned() + &str)
}

pub fn format_public_key<P: sp_core::Pair>(public_key: PublicFor<P>) -> String {
    format!("0x{}", HexDisplay::from(&public_key.as_ref()))
}

pub fn format_seed<P: sp_core::Pair>(seed: SeedFor<P>) -> String {
    format!("0x{}", HexDisplay::from(&seed.as_ref()))
}

pub fn format_hex_key<P: sp_core::Pair>(public_key: PublicFor<P>) -> String {
    format!("0x{}", HexDisplay::from(&public_key.as_ref()))
}

// pub fn pair_signer(pair: Pair) -> PairSigner<WeteeConfig, Pair> {
//     PairSigner::new(pair).try_into().unwrap()
// }
