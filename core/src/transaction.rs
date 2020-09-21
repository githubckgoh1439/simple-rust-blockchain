use serde_derive::{Deserialize, Serialize};

use crate::Address;
use crate::Signatures;


/// Represents a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    //nonce: u32,                     // Transaction's Nonce : prevent tx-replay purpose   
    sender: Address,
    recipient: Address,
    amount: u64,
    signature: Signatures,          // signer's signed signature (by keypairs that assigned)

}

impl Transaction {
    pub fn new (sender: Address, recipient: Address, amount: u64, signature: Signatures) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            signature,
        }
    }

    pub fn is_valid_transaction(&self) -> bool {
        // match (self.sender, self.signature) {
        //     (Some(p), Some(s)) if p.verify(&self.calculate_hash(), &s).is_ok() => true,
        //     (None, _) => true, // For miner reward
        //     _ => false,
        // }
        return true;
    }

    // pub fn get_sender (&self) -> &Option<PublicKey> {
    //     return &self.sender;
    // }

    // pub fn get_recipient (&self) -> &Option<PublicKey> {
    //     return &self.recipient;
    // }

}

