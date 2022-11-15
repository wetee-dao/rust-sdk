use bip39::{Language, Mnemonic, MnemonicType};

pub fn generate()-> String{
   let words = MnemonicType::for_word_count(12).unwrap();
   let mnemonic = Mnemonic::new(words, Language::English);
   println!("{}",mnemonic.to_string());
   return mnemonic.to_string()
}