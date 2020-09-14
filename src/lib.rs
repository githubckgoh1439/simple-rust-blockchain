
extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

type Address = String;


mod block;
pub use crate:: block::Block;
mod transaction;
pub use crate::transaction::Transaction;
pub use crate::block::blockheader::Blockheader;
mod blockchain;
pub use crate:: blockchain::Chain;


use std::time::{ SystemTime, UNIX_EPOCH };

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}


