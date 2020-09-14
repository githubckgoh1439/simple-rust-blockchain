extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate blake3;
extern crate primitive_types;
extern crate serde_cbor;


use self::sha2::{Sha256, Digest};
use std::fmt::Write;

use super::*;

#[derive(Debug)]
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

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u64, signatures: String) -> bool {

        self.current_transaction.push(Transaction::new(sender, recipient, amount, signatures));
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };

        Chain::hash(&block.get_blockheader())
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: u64) -> bool {
        self.mining_reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {

        let sender = String::from("Root");
        let recipient =  self.miner_address.clone();
        let amount = self.mining_reward;
        let genesis_signature = String::from("signature_for_genesis");
        
        let reward_transaction = Transaction::new(sender, recipient, amount, genesis_signature);

        let nonce = 0;
        let timestamp =  now();
        let pre_hash = self.last_hash();
        let difficulty = self.difficulty;

        let mut block = Block::new(nonce, timestamp, vec![], pre_hash, difficulty);


        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);
        block.header.set_merkle_root_hash(Chain::get_merkle(block.transactions.clone()));
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(current_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &current_transaction {
            let hash = Chain::hash(t);
            //let hash = Chain::blake3_hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            //let nh = Chain::blake3_hash(&h1);
            merkle.push(nh);
        }

        merkle.pop().unwrap()
    }

    // refer : [pub fn mine(&mut self) {}]
    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            //let hash = Chain :: blake3_hash(header);
            let slice = &hash[..header.get_difficulty() as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        let idx = header.get_nonce() + 1;
                        header.set_nonce(idx);
                    } else {
                        println!("[proof_of_work HERE !!!] == Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    let idx = header.get_nonce() + 1;
                    header.set_nonce(idx);
                continue;
                },
            };
        }

        println!("Block Mined");
    }
    
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }

    pub fn is_valid_chain(&self) -> bool {
        let blocks = &self.chain;

        for (i, block) in blocks.iter().enumerate() {
            // if i > 0 && block.get_blockheader().get_pre_hash() != blocks[i - 1].get_blockheader().get_merkle_root_hash() {
            //     return false;
            // }

            if !block.has_valid_transactions() {
                return false;
            }
        }

        return true;    }
}
