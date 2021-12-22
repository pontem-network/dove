// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
// use move_package::source_package::parsed_manifest::SourceManifest;
//
// /// Get a hash object in the form of u64
// pub trait HashU64: Hash {
//     /// Conversion to Bytes
//     fn hash_u64(&self) -> u64 {
//         let mut hasher = DefaultHasher::new();
//         self.hash(&mut hasher);
//         hasher.finish()
//     }
// }
//
// impl HashU64 for SourceManifest {}
