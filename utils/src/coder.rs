// use serde::{Serialize,Deserialize};
// use bincode;
// use crypto::sha3::Sha3;
// use crypto::digest::Digest;

// pub fn unmarshal<'a,T>(bytes: &'a [u8]) -> T
//     where T: Deserialize<'a>
// {
//     bincode::deserialize(bytes).unwrap()
// }

// pub fn marshal<T: ?Sized>(val: &T) ->Vec<u8>
//     where T: Serialize
// {
//     bincode::serialize(val).unwrap()
// }

// pub fn hash(data: &[u8]) -> String {
//     let mut had = Sha3::sha3_256();
//     had.input(data);
//     had.result_str()
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[derive(Debug,Deserialize,Serialize)]
//     struct Point {
//         x: i32,
//         y: String,
//     }

//     #[test]
//     fn test_mush() {
//         let a = Point{x:212,y:String::from("Abc")};
//         let a1 = marshal(&a);
//         println!("a1: {}",String::from_utf8_lossy(&a1));

//         let a2:Point = unmarshal(&a1);
//         println!("a2: {:#?}",a2);
//     }

// }


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

