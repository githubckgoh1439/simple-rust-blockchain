
use serde_derive::{Deserialize, Serialize};

use crate::Address;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    address: Address,
    balance: u64,
    sequence: u64,
    number : u64,
    flag: bool,
}

impl Account {
    pub fn new(addr: Address, bal:u64, seq:u64,num:u64,flag:bool) -> Self {
        Account {
            address: addr,
            balance: bal,
            sequence: seq,
            number:num,
            flag:flag,
        }
    }
}


