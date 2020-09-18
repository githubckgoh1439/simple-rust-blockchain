
use std::io;
use std::process;
use std::io::Write;

use core::blockchain;

// refer : 
// 1. https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs

fn main() {

    let miner_address = String::from("root-2");
    let difficulty = String::from("2").trim().parse::<u32>().expect("need INT");
    let mut choice = String::new();
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(),difficulty);


    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("5) Finalised the Chain !!!");
        println!("0) Exit");
        print!("enter your choice: ");

        io::stdout().flush();
        choice.clear();
        
        io::stdin().read_line(&mut choice);
        println!();

        match choice.trim().parse().unwrap() {
            0 => 
            {
                println!("exiting!");
                process::exit(0);
            },
            1 =>            
            {
                //3.1 created a new transaction object
                //3.2 added this transaction object into Block object 
                let mut amount = String::from("155");
                chain.new_transaction(String::from("coni-blake3"), String::from("ceni-blake3"), amount.trim().parse().unwrap(), String::from("signature"));
                println!("transaction added : 1st");

                amount = String::from("22");
                chain.new_transaction(String::from("AA-blake3"), String::from("BB-blake3"), amount.trim().parse().unwrap(), String::from("signature"));
                println!("transaction added : 2nd");

                amount = String::from("37");
                chain.new_transaction(String::from("CC-blake3"), String::from("DD-blake3"), amount.trim().parse().unwrap(), String::from("signature"));
                println!("transaction added : 3rd");

                amount = String::from("119");
                chain.new_transaction(String::from("mdemir-blake3"), String::from("ali-blake3"), amount.trim().parse().unwrap(), String::from("signature"));
                println!("transaction added : 4th");
                
            },
            2 =>
            {
                //4. added this Block object into the CHAIN list
                //4.1 set Block-hash using method of 'crypto_hash::Algorithm::SHA256'
                //4.2 mining the block
                //4.3 added this mined-block into the CHAIN list
                chain.generate_new_block();    
                println!("block generated successfully");    
            },
            3 =>
            {
                let mut new_difficulty = String::new();
                print!("enter new difficulty");
                io::stdout().flush();
                io::stdin().read_line(&mut new_difficulty);
                let res = chain.update_difficulty(
                    new_difficulty.trim().parse().unwrap()
                );

                match res {
                    true => println!("updated difficulty"),
                    false => println!("failed to update difficulty"),
                }
            },
            4 =>
            {
                let mut new_reward = String::new();
                print!("enter new reward");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(
                    new_reward.trim().parse().unwrap()
                );

                match res {
                    true => println!("updated reward"),
                    false => println!("failed to update reward"),
                }
            },
            5 =>
            {
                println!("Chain is VALID ? : {}", chain.is_valid_chain());
                println!("start ====================== THE CHAIN !!! ======================");
                println!("{:#?}", chain);
                println!("end  ====================== THE CHAIN !!! ====================== ");
            },
            _ => println!("invalid option, please retry"),
        }
    }
}





