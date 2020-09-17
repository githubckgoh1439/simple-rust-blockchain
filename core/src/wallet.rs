use rand::rngs::{OsRng};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
//https://github.com/input-output-hk/jormungandr/issues/2461


#[derive(Debug)]
pub struct Wallet {
    pub public_key: PublicKey,
    pub private_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        Wallet {
            public_key: keypair.public,
            private_key: keypair.secret,
        }
    }
}

