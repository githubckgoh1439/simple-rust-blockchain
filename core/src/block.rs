
use serde_derive::{Deserialize, Serialize};
use primitive_types::{H256};

use crate::blockheader::Blockheader;
use crate::transaction::Transaction;
use utils::hash::blake3_hash;


/// Represents a Block
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Blockheader,
    tx_count: u32,                                 // number of transactions data under this Block [https://github.com/dollarkillerx/Simple-blockchain/tree/master/]
    pub transactions: Vec<Transaction>,         // transactions data [https://github.com/dollarkillerx/Simple-blockchain/tree/master/]
}


impl Block {
    pub fn new (block_no: u64, nonce: u32, timestamp: u128, transactions: Vec<Transaction>, previous_hash: H256, difficult_level: u32) -> Block {

        let header = Blockheader::new(block_no, nonce, timestamp, previous_hash, difficult_level);
        let block = Block {
            header,
            tx_count: 0,
            transactions: transactions,
        };
        return block;
    }

    pub fn get_blockheader(&self) -> Blockheader {
        return self.header.to_owned();
    }

    pub fn set_count(&mut self, cn: u32 ){
        self.tx_count = cn;
    }

    // get the previous-Block's block-number
    pub fn get_last_block_number(&mut self, header: &mut Blockheader) -> u64{
        let block_no = header.get_block_number().to_owned();
        // println!("[last_hash !!!] == pre Block hash [header.merkle_root_hash] : {:?}", hash);
        return block_no;

    }
    

    // get the previous-Block-hash-value
    pub fn get_last_hash(&mut self, header: &mut Blockheader) -> H256{
        let hash = header.get_merkle_root_hash().to_owned();
        return hash;

    }

    // return the Hash-value of current-block & set to 'merkle_root_hash'
    pub fn get_current_hash(&mut self, header: &mut Blockheader) -> H256{
        let hash = blake3_hash(header);
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


    // //sample-1 : PoW
    // pub fn proof_of_work(&mut self, header: &mut Blockheader) {

    //     let mut s = String::new();
    //     for _i in 0..DIFFICULT_LEVEL {
    //         s.push_str("0");
    //     }
    //     println!("[proof_of_work] == s.push_str(0) : {:?}", s);
    //     let target = s;
    //     while header.get_merkle_root_hash().to_string()[..DIFFICULT_LEVEL as usize] != target {
    //         header.set_nonce(header.get_nonce() +  1);

    //         let current_hash = self.get_current_hash(&mut self.header.to_owned());
    //         header.set_merkle_root_hash(current_hash);

    //     }

    //     println!("Block Mined");
    // }


    //sample-2 : PoW
    // pub fn proof_of_work(&mut self, header: &mut Blockheader) {
    //     loop {
    //         let hash = coder::blake3_hash(header).to_string();
    //         let slice = &hash[..header.get_difficulty() as usize];

    //         match slice.parse::<u32>() {
    //             Ok(val) => {
    //                 if val != 0 {
    //                     let idx = header.get_nonce() + 1;
    //                     header.set_nonce(idx);
    //                 } else {
    //                     println!("[proof_of_work HERE !!!] == current Block hash: {}", hash);
    //                     break;
    //                 }
    //             },
    //             Err(_) => {
    //                 let idx = header.get_nonce() + 1;
    //                 header.set_nonce(idx);
    //             continue;
    //             },
    //         };
    //     }
    //     println!("Block Mined");
    // }

}

