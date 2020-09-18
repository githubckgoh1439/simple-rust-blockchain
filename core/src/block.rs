
use serde_derive::{Deserialize, Serialize};
use primitive_types::{H256};

use crate::blockheader::Blockheader;
use crate::transaction::Transaction;
use utils::coder;


/// Represents a Block
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Blockheader,
    count: u32,                                 // number of transactions data under this Block [https://github.com/dollarkillerx/Simple-blockchain/tree/master/]
    pub transactions: Vec<Transaction>,         // transactions data [https://github.com/dollarkillerx/Simple-blockchain/tree/master/]
}


impl Block {
    pub fn new (index: u32, timestamp: u128, transactions: Vec<Transaction>, previous_hash: H256, difficult_level: u32) -> Block {

        let header = Blockheader::new(index, timestamp, previous_hash, difficult_level);
        let block = Block {
            header,
            count: 0,
            transactions: transactions,
        };
        return block;
    }

    pub fn get_blockheader(&self) -> Blockheader {
        return self.header.to_owned();
    }

    pub fn set_count(&mut self, cn: u32 ){
        self.count = cn;
    }


    // get the previous-Block-hash-value
    pub fn get_last_hash(&mut self, header: &mut Blockheader) -> H256{
        let hash = header.get_merkle_root_hash().to_owned();
        println!("[last_hash !!!] == pre Block hash [header.merkle_root_hash] : {:?}", hash);
        return hash;

    }

    // return the Hash-value of current-block & set to 'merkle_root_hash'
    pub fn get_current_hash(&mut self, header: &mut Blockheader) -> H256{
        let hash = coder::blake3_hash(header);
        println!("[proof_of_work HERE !!!] == current Block hash: {:?}", hash);
        return hash;

    }

    pub fn has_valid_transactions(&self) -> bool {
        for tran in &self.transactions {
            if !tran.is_valid_transaction() {
                return false;
            }
        }

        return true;
    }

}


