

use super::*;


/// Represents a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: Address,
    recipient: Address,
    amount: u64,
    signature: String,

}

impl Transaction {
    pub fn new (sender: Address, recipient: Address, amount: u64, signature: String) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            signature,
        }
    }


    pub fn is_valid_transaction(&self) -> bool {
        return true;
    }
}



