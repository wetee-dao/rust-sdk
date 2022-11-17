use std::{collections::HashMap};
use bip39::{Language, Mnemonic, MnemonicType};
use sp_core::hexdisplay::AsBytesRef;
use subxt::{
   ext::{
     sp_core::{sr25519::{Pair}, Pair as TraitPair,hexdisplay::HexDisplay,crypto::{Ss58AddressFormat,Ss58Codec}},
   }
};
use xsalsa20poly1305::{aead::{generic_array::GenericArray, Aead}, XSalsa20Poly1305, KeyInit};
use crate::model::account::*;

/// 公钥类型
pub type PublicFor<P> = <P as sp_core::Pair>::Public;
/// 私钥类型
pub type SeedFor<P> = <P as sp_core::Pair>::Seed;
/// 加密噪点
const NONCE: &[u8; 24] = &[
    0x69, 0x69, 0x6e, 0xe9, 0x55, 0xb6, 0x2b, 0x73, 0xcd, 0x62, 0xbd, 0xa8, 0x75, 0xfc, 0x73, 0xd6,
    0x82, 0x19, 0xe0, 0x03, 0x6b, 0x7a, 0x0b, 0x37,
];

pub fn generate()-> String {
   let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
   println!("{}",mnemonic.phrase().to_string());
   return mnemonic.phrase().to_string()
}

// {
// 	"encoded": "RdDDbQWGHOPzn+J0sbuVYwPcXomNgiY0FOjHwURZyOkAgAAAAQAAAAgAAAAC2bhNHz9ON63/Ar5Vca+aeslC76i0w2BfeO7wpYvPFGVFPf1tYWpcH/QtjpNfhafV1b2ElB+QSW7oD2AHm823T/zlvKzj1tc+qamn0vxWN9dpI5LsW15iXT3/oMcm5wicBf043qnH0ui8+bE/zwtDvkQ1Wvf11gP3e1fUJ/c+0uncVrfZfwMNw8rWNvu3/+ZO++TIR1lSuyWpHw0R",
// 	"encoding": {
// 		"content": ["pkcs8", "sr25519"],
// 		"type": ["scrypt", "xsalsa20-poly1305"],
// 		"version": "3"
// 	},
// 	"address": "5Epkom3bMNTcU4W47qCdjPmaAzkoRFYfJwZBorgqG9EpsHPb",
// 	"meta": {
// 		"genesisHash": "",
// 		"name": "测试rust seed",
// 		"whenCreated": 1668598490767
// 	}
// }

pub fn get_seed_phrase(seed_str: String,name: String,password: String) -> anyhow::Result<KeringJSON> {
   // 助记词换账户
   let (pair, seed) =  Pair::from_phrase(&seed_str, None).unwrap();

   // 获取公钥
   let public_key = pair.public();
   println!("Secret phrase:  {}",seed_str);
   println!("Secret   seed:  {}",format_seed::<Pair>(seed));
   println!("Public    key:  {}",format_public_key::<Pair>(public_key.clone()));
   println!("SS58  Address:  {}",public_key.to_ss58check_with_version(
      Ss58AddressFormat::custom(42),
   ));

   // 因key必须32位，即重复key得到32位key
   let pwb = password.as_bytes().to_vec();
   let pwb_len = pwb.len();
   let mut keypw = vec![];
   if pwb_len == 32 {
      keypw = password.as_bytes().to_vec();
   }else{
      for i in 0..32 {
         let index = i%pwb_len;
         keypw.push(pwb[index]);
      }
   }
   let key = GenericArray::from_slice(keypw.as_bytes_ref());
   let nonce = GenericArray::from_slice(NONCE);

   // 获取加密对象
   let cipher = XSalsa20Poly1305::new(key);

   // 加密文本
   let ciphertext = cipher.encrypt(nonce, seed.as_slice()).unwrap();

   // let mut ciphertext2 = ciphertext.clone();
   // // 解码
   // let seed2 = cipher.decrypt(nonce, ciphertext2.as_slice()).unwrap();
   // let p3 = Pair::from_seed_slice(&seed2.as_slice()).unwrap();
   // let public_key3 = p3.public();
   // println!("Public333 key:  {}",format_public_key::<Pair>(public_key3.clone()));

   // 账户元数据
   let mut meta: HashMap<String,String> = HashMap::new();
   meta.insert("name".to_string(), name);

   Ok(KeringJSON{
      address: public_key.to_ss58check_with_version(
         Ss58AddressFormat::custom(42),
      ),
      encoded: hex::encode(ciphertext),
      encoding: KeringJSONEncoding{
         content: vec!["sr25519".to_string()],
         typex: "xsalsa20-poly1305".to_string(),
         version: "asyou-0".to_string(),
      },
      meta: meta,
   })
}


fn format_public_key<P: sp_core::Pair>(public_key: PublicFor<P>) -> String {
	format!("0x{}", HexDisplay::from(&public_key.as_ref()))
}

pub fn format_seed<P: sp_core::Pair>(seed: SeedFor<P>) -> String {
	format!("0x{}", HexDisplay::from(&seed.as_ref()))
}