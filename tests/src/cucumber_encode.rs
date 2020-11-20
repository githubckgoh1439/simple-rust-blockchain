
use async_trait::async_trait;
use std::borrow::Cow;
use std::convert::Infallible;

// use utils::hash::blake3_hash;

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;
use std::fs::File;
use std::io::Write;
use std::fs;

struct Encryptor<'a> {
    pub input: Cow<'a, str>,
    pub key: &'a [u8; 32],
    pub nonce: &'a [u8; 12],
}

impl<'a> Encryptor<'a> {
    pub fn write_encrypted(&self) {

        let filepath = "../test_encode_decode.txt";

        let cipher_key = GenericArray::from_slice(self.key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = GenericArray::from_slice(self.nonce);

        let encrypted = cipher
            .encrypt(nonce, self.input.as_bytes())
            .expect("Encryption failed.");

        let mut file = File::create(filepath).expect("Could not create the test file.");
        file.write(encrypted.as_slice())
            .expect("Could not write the encrypted data to testfile.txt.");

        println!("testfile.txt written.")


    }
}


struct Decryptor<'a> {
    pub file_path: &'a str,
    pub key: &'a [u8; 32],
    pub nonce: &'a [u8; 12],
 
}

impl<'a> Decryptor<'a> {
    pub fn read_decrypted(&self) -> String {
        let cipher_key = GenericArray::from_slice(self.key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = GenericArray::from_slice(self.nonce);

        let filepath = "../test_encode_decode.txt";
        let encrypted = fs::read(filepath).expect("Could not encrypted message file.");

        let decrypted = cipher
            .decrypt(nonce, encrypted.as_ref())
            .expect("Could not decrypt the encrypted message.");
        String::from_utf8(decrypted).expect("Could not convert the decrypted bytes into String.")



    }
}


pub struct EncodeWorld {
    encryptor: Encryptor<'static>,
    decryptor: Decryptor<'static>,
    encrypted: String,
    decrypted: String,
}

#[async_trait(?Send)]
impl cucumber::World for EncodeWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let key = &[1; 32];
        let nonce = &[3; 12];
        let filepath = "../test_encode_decode.txt";


        Ok(Self {
            encryptor: Encryptor {
                input: Cow::Borrowed(""),
                key,
                nonce,
            },
            decryptor: Decryptor {
                file_path: filepath,
                key,
                nonce,
            },
            encrypted: "".to_string(),
            decrypted: "".to_string(),
        })

    }
}



mod example_steps {
    use cucumber::{Steps, t};
    use std::fs;
    use std::path::Path;
    use std::borrow::Cow;

    
pub fn steps() -> Steps<crate::EncodeWorld> {
    let mut builder: Steps<crate::EncodeWorld> = Steps::new();

    builder
        .given_regex_async(
            r#"^I have an Encoder initialized with input "([\w\s!]+)"$"#,
            t!(|mut world, texts_to_encrypt, _step| {
                world.encryptor.input = Cow::Owned(texts_to_encrypt[1].to_owned());
                world
            }),
        )
        .then_regex_async(
            r#"^I should see "([\w\s!]+)" in the Encoder's input field$"#,
            t!(|world, expected_texts, _step| {
                assert_eq!(expected_texts[1], world.encryptor.input);
                world
            }),
        )
        .when_async(
            "I Encode the Encoder's input",
            t!(|world, _step| {
                world.encryptor.write_encrypted();
                world
            }),
        )
        .then_async(
            "testfile.txt exists",
            t!(|_world, _step| {
                let testfile_path = Path::new("../test_encode_decode.txt");
                assert_eq!(testfile_path.exists(), true);
                _world
            }),
        )
        .then_async(
            "testfile.txt is not empty",
            t!(|mut world, _step| {
                let enc_message = fs::read("../test_encode_decode.txt").expect("Could not read test file.");
                world.encrypted = base64::encode(&enc_message);

                assert_eq!(world.encrypted.len() > (0 as usize), true);
                world
            }),
        )
        .when_async(
            "I decrypt testfile.txt",
            t!(|mut world, _step| {
                world.decrypted = world.decryptor.read_decrypted();
                world
            }),
        )
        .then_regex_async(
            r#"^the Decoded result should be "([\w\s!]+)"$"#,
            t!(|mut world, expected_texts, _step| {
                assert_eq!(expected_texts[1], world.decrypted);
                world
            }),
        );

        return builder
    }
}


fn main() {

    let runner = cucumber::Cucumber::<EncodeWorld>::new()
        .features(&["../tests/features/example_encode/"])
        .steps(example_steps::steps());

    futures::executor::block_on(runner.run());
}
