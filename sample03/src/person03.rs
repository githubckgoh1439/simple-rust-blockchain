
use serde_derive::{Deserialize, Serialize};
use primitive_types::{H256};

use std::error::Error;
use std::fs::File;
use utils::hash::blake3_hash;


//20200929
/// Represents a Person02 Object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person03{
    name: String,
    age: u64,
    can_walked: bool    
}

impl Person03 {
    pub fn new (name: String, age: u64, walked: bool) -> Self {
        Person03 {
            name: name,
            age: age,
            can_walked: walked,
        }
    }

    pub fn get_name (&self) -> &str {
        return &self.name;
    }

    pub fn get_age (&self) -> u64 {
        return self.age;
    }

    pub fn can_walked (&self) -> bool {
        return self.can_walked;
    }

    pub fn without_blake3hash(&mut self) -> Result<bool, Box<dyn Error>>{

        //=====================================1. Write to File & Read File
        let person03_serialize_file = File::create("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample03/src/cbor/testing.cbor")?;
        serde_cbor::to_writer(person03_serialize_file, self)?;
    
        //=====================================2. Read File
        let person03_deserialize_file = File::open("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample03/src/cbor/testing.cbor")?;
        let rs_deserialize: Person03 = serde_cbor::from_reader(person03_deserialize_file)?;
        println!("[without_blake3hash()] --------- {:?}", rs_deserialize);
        println!("[without_blake3hash()] ---------name: {:?} == age: {:?} == IsWalked :{:?}", rs_deserialize.get_name(), rs_deserialize.get_age(), rs_deserialize.can_walked());
    
        return Ok(true);
    
    }
    
    // https://www.tutorialspoint.com/rust/rust_error_handling.htm
    pub fn use_blake3hash(&mut self)-> Result<bool, Box<dyn Error>>{
    
        let blake3_encode = blake3_hash(self);
        println!("[use_blake3hash() : encode] --------- {:?}", &blake3_encode);
    
        //=====================================1. Write to File & Read File
        let person03_serialize_file = File::create("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample03/src/cbor/testing.cbor")?;
        serde_cbor::to_writer(person03_serialize_file, &blake3_encode)?;
    
        //=====================================2. Read File
        let person03_deserialize_file = File::open("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample03/src/cbor/testing.cbor")?;
        let blake3_decode: H256 = serde_cbor::from_reader(person03_deserialize_file)?;
        println!("[use_blake3hash() : decode] --------- {:?}", blake3_decode);    
        
        return Ok(true);
    }
    
}


// ***** Option-1. Below can be used : If wish to implement in another outside file of 'person03_test.rs' 
#[cfg(test)]
#[path = "./person03_test.rs"]
mod person03_test;

