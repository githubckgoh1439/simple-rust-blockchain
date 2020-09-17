
extern crate serde_derive;
extern crate serde_cbor;
extern crate rand;
extern crate ed25519_dalek;
extern crate blake3;

pub mod blockchain;
pub mod block;
pub mod blockheader;
pub mod transaction;
pub mod account;
pub mod wallet;


pub type Address = String;
pub type Signatures = String;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

//=====================================================

