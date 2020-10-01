

use blake3;
use primitive_types::{H256};
use serde_cbor::{to_vec};

use std::time::{ SystemTime, UNIX_EPOCH };

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}



// path : sample02
/// https://crates.io/crates/blake3
/*
sample-1 : 0x07426472edd848a38da37e5466dc03054d307fe359a788af0fdde04a327e0c8a
sample-2 : 0xfbc99ec9cb52d3749eb0564917ae4f1a188cd572d2e06c5512ab9f34c5dd1ed5
sample-3 : 0x1f693ac7b96db638cb6386b7de595e88955aa8725351c11f6d12471eb7b17169
sample-4 : 0x4aa69f6db3b4b1456f56b06515daf7db8cfb57cd99be115b1fa164a8a23257a6
*/
pub fn blake3_hash<T: serde::Serialize>(item: &T)-> H256 {
    let bytes = to_vec(&item).unwrap();
    let mut hasher = blake3::Hasher::new();
    hasher.update(&bytes);
    let mut hash_output = [0; 32];
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut hash_output);
    H256(hash_output)
}

