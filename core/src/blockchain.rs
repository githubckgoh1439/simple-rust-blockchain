
use primitive_types::{H256};
use utils::coder;

use crate::block::Block;
use crate::transaction::Transaction;

use crate::Address;
use crate::Signatures;


#[derive(Debug, Clone)]
pub struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_address: Address,
    mining_reward: u64,
}


impl Chain {
    pub fn new(miner_address: Address, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
            difficulty,
            miner_address,
            mining_reward: 100,
        };

        chain.generate_genesis_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u64, signatures: Signatures) -> bool {
        self.current_transaction.push(Transaction::new(sender, recipient, amount, signatures));
        true
    }
    
   
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: u64) -> bool {
        self.mining_reward = reward;
        true
    }

    pub fn generate_genesis_block(&mut self) -> bool {

        // compile : genesis-transaction
        let sender = String::from("Root");
        let recipient =  self.miner_address.clone();
        let amount = self.mining_reward;
        let genesis_signature = String::from("signature_for_genesis");
        let reward_transaction = Transaction::new(sender, recipient, amount, genesis_signature);

        // compile : genesis-block for above genesis-transaction 
        let nonce = 0;
        let timestamp = coder::now();
        let pre_hash = H256([0; 32]);
        let difficulty = self.difficulty;

        // added this genesis-block into the Chain 
        let mut block = Block::new(nonce, timestamp, vec![reward_transaction], pre_hash, difficulty);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);

        // generate the Hash-value of this Block & set to 'merkle_root_hash'
        let current_hash = block.get_current_hash(&mut block.header.to_owned());
        block.header.set_merkle_root_hash(current_hash);

        // added this genesis-block into the Chain 
        // println!("{:#?}", &block);
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_genesis_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                
    }


    pub fn generate_new_block(&mut self) -> bool {

        // compile : non-genesis-block for current non-genesis-transaction 
        let nonce = 0;
        let timestamp = coder::now();
        let pre_hash = H256([0; 32]);
        let difficulty = self.difficulty;

        // added this non-genesis-block into the Chain 
        let mut block = Block::new(nonce, timestamp, vec![], pre_hash, difficulty);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);

        let mut block2 = block.clone();
        match self.chain.last() {
            Some(pre_block)=> block.header.set_pre_hash(block2.get_last_hash(&mut pre_block.get_blockheader())),
            None => block.header.set_pre_hash(H256([0; 32])), 
        }

        // generate the Hash-value of this Block & set to 'merkle_root_hash'
        let current_hash = block.get_current_hash(&mut block.header.to_owned());
        block.header.set_merkle_root_hash(current_hash);

        // added this genesis-block into the Chain 
        // println!("{:#?}", &block);
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_new_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true
    }
   
    pub fn is_valid_chain(&self) -> bool {
        let blocks = &self.chain;

        for (i, block) in blocks.iter().enumerate() {
            if i > 0 && block.get_blockheader().get_pre_hash() != blocks[i - 1].get_blockheader().get_merkle_root_hash() {  //by validate currnt-block-hash vs. previous-block-hash
                return false;
            }
            if !block.has_valid_transactions() {            //by validate signer's signature, using 'ed25519_dalek::{Verifier};'
                return false;
            }
        }

        return true;
    }


}


