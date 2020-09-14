
use super::*;


/// Represents a Block-header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockheader {
    nonce: u32,
    timestamp: u128,
    pre_hash: String,
    merkle_root_hash: String,
    difficulty: u32,
}


impl Blockheader {

    pub fn new (index: u32, timestamp: u128, previous_hash: String, difficult_level: u32) -> Self {

        let header = Blockheader {
            nonce: index,
            timestamp: timestamp,
            pre_hash: previous_hash,
            merkle_root_hash: String::new(),
            difficulty: difficult_level,
        };

        return header;
    }

    pub fn get_difficulty(&mut self) -> u32 {
        return self.difficulty;
    }

    pub fn get_nonce(&self) -> &u32 {
        return &self.nonce;
    }

    pub fn set_nonce(&mut self, index: u32){
        self.nonce = index;
    }

    pub fn set_merkle_root_hash(&mut self, mk: String ){
        self.merkle_root_hash = mk;
    }

    pub fn get_pre_hash(&self) -> &String {
        return &self.pre_hash;
    }

    pub fn get_merkle_root_hash(&self) -> &String {
        return &self.merkle_root_hash;
    }
    
}

