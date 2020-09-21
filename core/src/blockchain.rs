
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
    difficulty: u32,                        // used by consensus - PoW 
    miner_address: Address,                 // used by consensus - PoW
    mining_reward: u64,                     // used by consensus - PoW
}


impl Chain {
    pub fn new(miner_address: Address, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
            difficulty: difficulty,
            miner_address: miner_address,
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
        let block_no = 0;
        let nonce = 0;
        let timestamp = coder::now();
        let pre_hash = H256([0; 32]);
        let difficulty = String::from("1").trim().parse::<u32>().expect("need INT");

        // added this genesis-block into the Chain 
        let mut block = Block::new(block_no, nonce, timestamp, vec![reward_transaction], pre_hash, difficulty);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);
        block.header.set_block_number(1);           // is 1st genesis-block 

        // generate the Hash-value of this Block & set to 'merkle_root_hash'
        let current_hash = block.get_current_hash(&mut block.header.to_owned());
        block.header.set_merkle_root_hash(current_hash);

        const DIFFICULT_LEVEL: u32 = 2;
        for _j in 0..DIFFICULT_LEVEL{
            block.header.set_nonce(block.header.get_nonce() +  1);

            let current_hash = block.get_current_hash(&mut block.header.to_owned());
            block.header.set_merkle_root_hash(current_hash);
        }
        println!("Block Mined");


        // added this genesis-block into the Chain 
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_genesis_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                
    }


    pub fn generate_new_block(&mut self) -> bool {

        // compile : non-genesis-block for current non-genesis-transaction
        let block_no = 0; 
        let nonce = 0;
        let timestamp = coder::now();
        let pre_hash = H256([0; 32]);
        let difficulty = self.difficulty;

        // added this non-genesis-block into the Chain 
        let mut block = Block::new(block_no, nonce, timestamp, vec![], pre_hash, difficulty);
        block.transactions.append(&mut self.current_transaction);
        block.set_count(block.transactions.len() as u32);


        let mut block2 = block.clone();
        match self.chain.last() {
            Some(pre_block)=> {
                   block.header.set_pre_hash(block2.get_last_hash(&mut pre_block.get_blockheader()));
                   block.header.set_block_number(block2.get_last_block_number(&mut pre_block.get_blockheader()) + 1);
            },  
            None => block.header.set_pre_hash(H256([0; 32])), 
        }

        // generate the Hash-value of this Block & set to 'merkle_root_hash'
        let current_hash = block.get_current_hash(&mut block.header.to_owned());
        block.header.set_merkle_root_hash(current_hash);

        // PoW consensus
        // // block.proof_of_work(&mut block.header.to_owned());
        // const DIFFICULT_LEVEL: u32 = 2;
        // for _j in 0..DIFFICULT_LEVEL{
        //     block.header.set_nonce(block.header.get_nonce() +  1);

        //     let current_hash = block.get_current_hash(&mut block.header.to_owned());
        //     block.header.set_merkle_root_hash(current_hash);
        // }
        // println!("Block Mined");

        // added this genesis-block into the Chain 
        self.chain.push(block);

        println!("==================================================================");
        println!("generate_new_block()......................completed !!!");
        println!("==================================================================");
        println!("                              ");
        true
    }

    //sample-2 : PoW
    /// Start mining process to solve puzzle
    /// Guess the nonce until miner finds a hash that
    /// matches the set difficulty level (say, 5 leading zeroes)
    // pub fn mine(&mut self) {
    //     let target = get_difficult_string();

    //     while &self.hash[..DIFFICULT_LEVEL as usize] != target {
    //         self.nonce += 1;
    //         self.hash = calculate_hash(
    //             &self.pre_hash,
    //             &self.transaction,
    //             &self.timestamp,
    //             &self.nonce,
    //         )
    //     }

    //     println!("Block Mined");
    // }

    //sample-1 : PoW
    // pub fn proof_of_work(header: &mut Blockheader) {
    //     loop {
    //         let hash = Chain::hash(header);
    //         let slice = &hash[..header.get_difficulty() as usize];

    //         match slice.parse::<u32>() {
    //             Ok(val) => {
    //                 if val != 0 {
    //                     let idx = header.get_nonce() + 1;
    //                     header.set_nonce(idx);
    //                 } else {
    //                     println!("[proof_of_work HERE !!!] == Block hash: {}", hash);
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
