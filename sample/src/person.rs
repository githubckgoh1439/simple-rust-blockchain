
use serde_derive::{Deserialize, Serialize};
use primitive_types::{H256};

use std::error::Error;
use std::fs::File;
use utils::coder;


/// Represents a Person Object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person{
    name: String,
    age: u64,
    can_walked: bool    
}

impl Person {
    pub fn new (name: String, age: u64, walked: bool) -> Self {
        Person {
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

    // /home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample/src/person.rs
    // https://www.tutorialspoint.com/rust/rust_error_handling.htm
    pub fn without_blake3hash(&mut self) -> Result<bool, Box<dyn Error>>{
    // pub fn without_blake3hash(&mut self) -> Result<bool, String>{

        //=====================================1. Write to File & Read File
        let person_serialize_file = File::create("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample/src/cbor/person_serialize.cbor")?;
        serde_cbor::to_writer(person_serialize_file, self)?;
    
        //=====================================2. Read File
        let person_deserialize_file = File::open("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample/src/cbor/person_serialize.cbor")?;
        let rs_deserialize: Person = serde_cbor::from_reader(person_deserialize_file)?;
        println!("[without_blake3hash()] --------- {:?}", rs_deserialize);
        println!("[without_blake3hash()] ---------name: {:?} == age: {:?} == IsWalked :{:?}", rs_deserialize.get_name(), rs_deserialize.get_age(), rs_deserialize.can_walked());
    
        return Ok(true);
    
    }
    
    // https://www.tutorialspoint.com/rust/rust_error_handling.htm
    pub fn use_blake3hash(&mut self)-> Result<bool, Box<dyn Error>>{
    //   pub fn use_blake3hash(&mut self) -> Result<bool, String>{
    
        let blake3_encode = coder::blake3_hash(self);
        println!("[use_blake3hash() : encode] --------- {:?}", &blake3_encode);
    
        //=====================================1. Write to File & Read File
        let person_serialize_file = File::create("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample/src/cbor/person_serialize.cbor")?;
        serde_cbor::to_writer(person_serialize_file, &blake3_encode)?;
    
        //=====================================2. Read File
        let person_deserialize_file = File::open("/home/ckgoh/gowork/src/rust.com/github.com/simple-rust-blockchain/sample/src/cbor/person_serialize.cbor")?;
        let blake3_decode: H256 = serde_cbor::from_reader(person_deserialize_file)?;
        println!("[use_blake3hash() : decode] --------- {:?}", blake3_decode);    
        
        return Ok(true);
    }
    
}


// ***** Option-1. Below can be used : If wish to implement in another outside file of 'person_test.rs' 
#[cfg(test)]
#[path = "./person_test.rs"]
mod person_test;


// ***** Option-2. Below can be used : If wish to implement in this same file of 'person.rs' 
// #[cfg(test)]
// mod tests_person{
//     use super::*;

//     #[test]
//     fn test_without_blake3hash(){
//         let mut person1 =  Person::new(String :: from("Pop"), 0, false);
//         let rs1 = person1.without_blake3hash().unwrap();
//         assert_eq!(true, rs1);
//     } 

//     #[test]
//     fn test_use_blake3hash(){
//         let mut person2 =  Person::new(String :: from("John-Lim"), 0, false);
//         let rs2 = person2.use_blake3hash().unwrap();
//         assert_eq!(true, rs2);
//     } 


    // #[test]
    // fn person_get_name4(){
    //     let person3 =  Person::new(String :: from("alex"), 0, false);
    //     assert_eq!("alex", person3.get_name());
    // } 

    // #[test]
    // fn person_get_age5(){
    //     let person3 =  Person::new(String :: from("alex"), 13, false);
    //     assert_eq!(13, person3.get_age());
    // } 

    // #[test]
    // fn person_can_walked6(){
    //     let person3 =  Person::new(String :: from("alex"), 13, true);
    //     assert_eq!(true, person3.can_walked());
    // } 


// }
