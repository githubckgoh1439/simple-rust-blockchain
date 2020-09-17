use serde_derive::{Deserialize, Serialize};

use crate::Address;
use crate::Signatures;


/// Represents a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    sender: Address,
    recipient: Address,
    amount: u64,
    signature: Signatures,          // signer's signed signature

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

    // pub fn bytes(&self) -> Vec<u8> {
    //     let mut bytes = vec![];
    //     if let Some(sender) = self.sender {
    //         bytes.extend(sender.as_bytes());
    //     }
    //     if let Some(recipient) = self.recipient {
    //         bytes.extend(recipient.as_bytes());
    //     }
    //     bytes.extend(&self.amount.to_ne_bytes());

    //     bytes
    // }

    // pub fn calculate_hash(&self) -> Vec<u8> {
    //     crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    // }

    // pub fn sign_transaction(&mut self, key: Keypair) {
    //     if let Some(p) = self.sender {
    //         if p != key.public {
    //             panic!("You can not sign other's transaction!!!")
    //         } else {

    //             self.signature = Some(key.sign(&self.calculate_hash()));
    //         }
    //     }
    // }

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



