// use codec::Encode;
// use frame_metadata::v14::StorageEntryMetadata;
// use frame_metadata::{StorageEntryType, StorageHasher};
// use scale_info::form::PortableForm;
// use sp_core::storage::StorageKey;
// use substrate_api_client::{GetStorageTypes, Metadata, MetadataError};

// pub fn storage_value_key(
//     &self,
//     pallet: &'static str,
//     storage_item: &'static str,
// ) -> Result<StorageKey, MetadataError> {
//     Ok(self
//         .pallet(pallet)?
//         .storage(storage_item)?
//         .get_value(pallet)?
//         .key())
// }

// pub fn storage_map_key<K: Encode>(
//     &self,
//     pallet: &'static str,
//     storage_item: &'static str,
//     map_key: K,
// ) -> Result<StorageKey, MetadataError> {
//     Ok(self
//         .pallet(pallet)?
//         .storage(storage_item)?
//         .get_map::<K>(pallet)?
//         .key(map_key))
// }

// pub fn storage_map_key_prefix(
//     &self,
//     pallet: &'static str,
//     storage_item: &'static str,
// ) -> Result<StorageKey, MetadataError> {
//     self.pallet(pallet)?
//         .storage(storage_item)?
//         .get_map_prefix(pallet)
// }

// pub fn storage_double_map_key_prefix<K: Encode, Q: Encode>(
//     meta: Metadata,
//     pallet: &'static str,
//     storage_item: &'static str,
//     first_double_map_key: K,
//     second_double_map_key: Q,
// ) -> Result<StorageKey, MetadataError> {
//     Ok(meta
//         .pallet(pallet)?
//         .storage(storage_item)?
//         .get_map_prefix(pallet)?)
// }

// pub trait GetDoubleStorageTypes {
//     fn storage_double_map_prefix<K: Encode, Q: Encode>(
//         &self,
//         module_prefix: String,
//         storage_prefix: String,
//         key1: K,
//         key2: Q,
//     ) -> Result<StorageKey, MetadataError>;
// }

// // impl GetDoubleStorageTypes for StorageEntryMetadata<PortableForm> {
// pub fn storage_double_map_prefix<K: Encode, Q: Encode>(
//     store: &StorageEntryMetadata<PortableForm>,
//     module_prefix: String,
//     storage_prefix: String,
//     key1: K,
//     key2: Q,
// ) -> Result<StorageKey, MetadataError> {
//     // let mut bytes = sp_core::twox_128(pallet_prefix.as_bytes()).to_vec();
//     // bytes.extend(&sp_core::twox_128(name.as_bytes())[..]);
//     // Ok(StorageKey(bytes))
//     match &store.ty {
//         StorageEntryType::Map { hashers, .. } => {
//             let hasher1 = hashers.get(0).ok_or(MetadataError::StorageTypeError)?;
//             let hasher2 = hashers.get(1).ok_or(MetadataError::StorageTypeError)?;
//             let mut bytes = sp_core::twox_128(module_prefix.as_bytes()).to_vec();
//             bytes.extend(&sp_core::twox_128(storage_prefix.as_bytes())[..]);
//             bytes.extend(key_hash(&key1, &hasher1));
//             // bytes.extend(key_hash(&key2, &hasher2));
//             Ok(StorageKey(bytes))
//         }
//         _ => Err(MetadataError::StorageTypeError),
//     }
// }
// }

// fn key_hash<K: Encode>(key: &K, hasher: &StorageHasher) -> Vec<u8> {
//     let encoded_key = key.encode();
//     match hasher {
//         StorageHasher::Identity => encoded_key.to_vec(),
//         StorageHasher::Blake2_128 => sp_core::blake2_128(&encoded_key).to_vec(),
//         StorageHasher::Blake2_128Concat => {
//             // copied from substrate Blake2_128Concat::hash since StorageHasher is not public
//             let x: &[u8] = encoded_key.as_slice();
//             sp_core::blake2_128(x)
//                 .iter()
//                 .chain(x.iter())
//                 .cloned()
//                 .collect::<Vec<_>>()
//         }
//         StorageHasher::Blake2_256 => sp_core::blake2_256(&encoded_key).to_vec(),
//         StorageHasher::Twox128 => sp_core::twox_128(&encoded_key).to_vec(),
//         StorageHasher::Twox256 => sp_core::twox_256(&encoded_key).to_vec(),
//         StorageHasher::Twox64Concat => sp_core::twox_64(&encoded_key)
//             .iter()
//             .chain(&encoded_key)
//             .cloned()
//             .collect(),
//     }
// }
