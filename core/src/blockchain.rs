
use blake3;
use primitive_types::{H256};
use serde_cbor::{to_vec};
use utils::coder;

use crate::block::Block;
use crate::transaction::Transaction;
use crate::blockheader::Blockheader;

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
    
    pub fn last_hash(&self) -> H256 {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return H256([0; 32]),
        };

        Chain::blake3_hash(&block.get_blockheader())
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

        let sender = String::from("Root");
        let recipient =  self.miner_address.clone();
        let amount = self.mining_reward;
        let genesis_signature = String::from("signature_for_genesis");
        let reward_transaction = Transaction::new(sender, recipient, amount, genesis_signature);

        let nonce = 0;
        let timestamp = coder::now();
        let pre_hash = self.last_hash();
        let difficulty = self.difficulty;

        let mut block = Block::new(nonce, timestamp, vec![], pre_hash, difficulty);

        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);

        //1.
        let current_hash = Chain::proof_of_work(&mut block.header);
        // println!("[current_hash HERE !!!] == Block hash: {:?}", current_hash);
        block.header.set_merkle_root_hash(current_hash);

        //2.
        // Chain::proof_of_work(&mut block.header);
        println!("{:#?}", &block);
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_genesis_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true
    }


    pub fn generate_new_block(&mut self) -> bool {

        let nonce = 0;
        let timestamp =  coder::now();
        let pre_hash = self.last_hash();
        let difficulty = self.difficulty;

        let mut block = Block::new(nonce, timestamp, vec![], pre_hash, difficulty);

        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);
        block.header.set_merkle_root_hash(Chain::get_merkle(block.transactions.clone()));

        //1.
        // let current_hash = Chain::proof_of_work(&mut block.header);
        // block.header.set_merkle_root_hash(current_hash);

        //2.
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_new_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true
    }

    fn get_merkle(current_transaction: Vec<Transaction>) -> H256 {
        let mut merkle = Vec::new();

        for t in &current_transaction {
            let hash = Chain::blake3_hash(t);
            merkle.push(hash);
        }

        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) -> H256{
        let hash = Chain::blake3_hash(header);
        println!("[proof_of_work HERE !!!] == Block hash: {:?}", hash);
        return hash;

    }

    ///YK - blake3
    /// https://crates.io/crates/blake3
    /*
    sample-1 : 0x07426472edd848a38da37e5466dc03054d307fe359a788af0fdde04a327e0c8a
    sample-2 : 0xfbc99ec9cb52d3749eb0564917ae4f1a188cd572d2e06c5512ab9f34c5dd1ed5
    sample-3 : 0x1f693ac7b96db638cb6386b7de595e88955aa8725351c11f6d12471eb7b17169
    sample-4 : 0x4aa69f6db3b4b1456f56b06515daf7db8cfb57cd99be115b1fa164a8a23257a6
    */
    pub fn blake3_hash<T: serde::Serialize>(item: &T)-> H256 {
        let bytes = to_vec(&item).unwrap();
        let mut hasher = blake3::Hasher::new();
        hasher.update(&bytes);
        let mut hash_output = [0; 32];
        let mut output_reader = hasher.finalize_xof();
        output_reader.fill(&mut hash_output);
        H256(hash_output)
    }

    pub fn is_valid_chain(&self) -> bool {
        let blocks = &self.chain;

        for (i, block) in blocks.iter().enumerate() {
            if !block.has_valid_transactions() {            //by validate signer's signature, using 'ed25519_dalek::{Verifier};'
                return false;
            }
        }

        return true;
    }

}


