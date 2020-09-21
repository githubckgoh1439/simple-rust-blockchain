
use serde_derive::{Deserialize, Serialize};
use primitive_types::{H256};

/// Represents a Block-header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockheader {
    block_number: u64,          // sequence number of the block   
    nonce: u32,                 // Block's Nonce : related to different consensus algo : eg. PoW, PoS, RAFT, BFT-Tendermint.....   
    timestamp: u128,
    pre_hash: H256,             // hash of the pre block 
    merkle_root_hash: H256,     // transaction merkle hash
    difficulty: u32,            // used by consensus - PoW
}


impl Blockheader {

    pub fn new (block_no: u64, nonce: u32, timestamp: u128, previous_hash: H256, difficult_level: u32) -> Self {

        let header = Blockheader {
            block_number: block_no, 
            nonce: nonce,
            timestamp: timestamp,
            pre_hash: previous_hash,
            merkle_root_hash: H256([0; 32]),
            difficulty: difficult_level,
        };

        return header;
    }

    pub fn get_difficulty(&mut self) -> u32 {
        return self.difficulty;
    }

    pub fn get_block_number(&self) -> &u64 {
        return &self.block_number;
    }

    pub fn set_block_number(&mut self, block_no: u64){
        self.block_number = block_no;
    }

    pub fn get_nonce(&self) -> &u32 {
        return &self.nonce;
    }

    pub fn set_nonce(&mut self, index: u32){
        self.nonce = index;
    }

    /// Set hash of current block
    pub fn set_merkle_root_hash(&mut self, mk: H256 ){
        self.merkle_root_hash = mk;
    }

    /// Set hash of previous block as pre hash
    pub fn set_pre_hash(&mut self, pre: H256) {
        self.pre_hash = pre;
    }

    pub fn get_pre_hash(&self) -> &H256 {
        return &self.pre_hash;
    }

    pub fn get_merkle_root_hash(&self) -> &H256 {
        return &self.merkle_root_hash;
    }
    
}

