
mod hash_test{
    use super::super::*;

    use serde_derive::*;
    use primitive_types::{H256};
    use std::fs::File;
    use std::fs::remove_file;
    
    #[derive(Debug)]
    pub enum PersonError {
        CheckForInvalidCondition(String),
        CheckForFileCreate(String),
        CheckForFileOpen(String),
        CheckForFileWriter(String),
        CheckForFileReader(String),
        CheckForFileRemove(String),
    
    }
    
    // Testing Object
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Person{
        name: String,
        age: u64,
        can_walked: bool    
    }

    impl Person {
        fn new (name: String, age: u64, walked: bool) -> Self {
            Person {
                name: name,
                age: age,
                can_walked: walked,
            }
        }

        fn get_name (&self) -> &str {
            return &self.name;
        }
    
        fn get_age (&self) -> u64 {
            return self.age;
        }
    
        fn validate(&self) -> Result<bool, PersonError> {
    
            if self.get_name().len() < 1{
                return Err(PersonError::CheckForInvalidCondition("Name must not be empty.".to_string()));
            }
    
            if self.get_age() < 1{
                return Err(PersonError::CheckForInvalidCondition("Invalid Age.".to_string()));
            }
    
            return Ok(true)
        }
    
        pub fn serialize_hashing_result(&self)-> Result<bool, PersonError>{

            if let Err(error) = self.validate() {
                return Err(error);
            }

            let encode = blake3_hash(self);
            let create_file = match File::create("./testing.cbor") {
                Err(err) => Err(PersonError::CheckForFileCreate(err.to_string())),
                Ok(file) =>{
                    Ok(file)
                },
            };

            if create_file.is_ok(){

                let write_file = match serde_cbor::to_writer(create_file.unwrap(), &encode){
                    Err(err) => Err(PersonError::CheckForFileWriter(err.to_string())),
                    Ok(()) => {
                        println!("serialize: {:?}", &encode);
                        Ok(())
                    },
                    
                };

                if write_file.is_ok(){
                      let open_file = match File::open("./testing.cbor") {
                        Err(err) => {
                            Err(PersonError::CheckForFileOpen(err.to_string()))
                        },             
                        Ok(file) => {
                            Ok(file)
                        },
                    };
                    
                    if open_file.is_ok(){
                            let read_file  = serde_cbor::from_reader(open_file.unwrap());
                            if read_file.is_ok(){
                                let val:H256 = read_file.unwrap();
                                println!("deserialize: {:?}", &val);    

                                return Ok(true);
                            }else{
                                return Err(PersonError::CheckForFileReader("Error while read file".to_string()));
                            }

                    }else{
                        return Err(PersonError::CheckForFileReader("Error while open file".to_string()));
                    }

                }else{
                    return Err(PersonError::CheckForFileWriter("Error while write file".to_string()));
                }

            }else{
                return Err(PersonError::CheckForFileCreate("Error while create file".to_string()));
            }

        }

    }

    //check for : valid object by hashing and cbor_serialized result  
    #[test]
    fn hash_serialize_one_valid_object(){

        let person =  Person::new(String :: from("bob"), 31, false);
        let rs = person.serialize_hashing_result();
        assert_eq!(rs.is_ok(), true);

        let rm = remove_file("./testing.cbor");
        if rm.is_err(){
            PersonError::CheckForFileRemove("Error while remove file".to_string());
        }

    } 


    //check for : invalid object attribute by hashing and cbor_serialized result  
    #[test]
    fn hash_serialize_invalid_name(){

        let person =  Person::new(String :: from(""), 22, true);
        let rs = person.serialize_hashing_result();
        assert_eq!(rs.is_err(), true);

        let rm = remove_file("./testing.cbor");
        if rm.is_err(){
            PersonError::CheckForFileRemove("Error while remove file".to_string());
        }
     
    } 

    //check for : invalid object attribute on cbor_serialized result  
    #[test]
    fn hash_serialize_invalid_age(){

        let person =  Person::new(String :: from("bob"), 0, false);
        let rs = person.serialize_hashing_result();
        assert_eq!(rs.is_err(), true);

        let rm = remove_file("./testing.cbor");
        if rm.is_err(){
            PersonError::CheckForFileRemove("Error while remove file".to_string());
        }
     
    } 


    //check for : List of valid objects by hashing and cbor_serialized result  
    #[test]
    fn hash_serialize_all_valid_objects(){

        let person1 =  Person::new(String :: from("alice"), 1, false);
        let person2 =  Person::new(String :: from("A"), 11, true);
        let person3 =  Person::new(String :: from("B"), 22, true);
        let person4 =  Person::new(String :: from("C"), 33, true);
        let person5 =  Person::new(String :: from("P"), 44, true);
        let person6 =  Person::new(String :: from("qqq"), 55, true);

        let mut v = Vec::new();
        v.push(person1);
        v.push(person2);
        v.push(person3);
        v.push(person4);
        v.push(person5);
        v.push(person6);

        for i in v {
            let rs = i.serialize_hashing_result();
            assert_eq!(rs.is_ok(), true);
        }

        let rm = remove_file("./testing.cbor");
        if rm.is_err(){
            PersonError::CheckForFileRemove("Error while remove file".to_string());
        }
    } 


    //check for : Some of invalid objects by hashing and cbor_serialized result  
    #[test]
    fn hash_serialize_some_invalid_objects(){

        let person1 =  Person::new(String :: from("alice"), 1, false);
        let person2 =  Person::new(String :: from("A"), 11, true);
        let person3 =  Person::new(String :: from("B"), 22, true);
        let person4 =  Person::new(String :: from("C"), 0, false);
        let person5 =  Person::new(String :: from(""), 44, true);
        let person6 =  Person::new(String :: from(""), 55, true);

        let mut p = Vec::new();
        p.push(person1);
        p.push(person2);
        p.push(person3);
        p.push(person4);
        p.push(person5);
        p.push(person6);

        let mut count = 0;
        for j in p {
            let rs = j.serialize_hashing_result();

            if count < 3{
                assert_eq!(rs.is_ok(), true);
            }else{
                assert_eq!(rs.is_err(), true);
            }
            count += 1;
        }

        let rm = remove_file("./testing.cbor");
        if rm.is_err(){
            PersonError::CheckForFileRemove("Error while remove file".to_string());
        }

    } 
    
}


