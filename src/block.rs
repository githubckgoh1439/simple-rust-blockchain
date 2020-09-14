
use super::*;

pub mod blockheader;
use block::blockheader::Blockheader;

/// Represents a Block
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Blockheader,
    count: u32,
    pub transactions: Vec<Transaction>,
}


impl Block {
    pub fn new (index: u32, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String, difficult_level: u32) -> Block {

        let header = Blockheader::new(index, timestamp, previous_hash, difficult_level);
        let block = Block {
            header,
            count: 0,
            transactions: transactions,
        };
        return block;
    }

    pub fn get_blockheader(&self) -> &Blockheader {
        return &self.header;
    }

    pub fn set_count(&mut self, cn: u32 ){
        self.count = cn;
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

