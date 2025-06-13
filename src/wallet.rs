use k256::ecdsa::{SigningKey, VerifyingKey, Signature, signature::{Signer, Verifier}};
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
let mut rng = rand::thread_rng();
let private_key = SigningKey::random(&mut rng);
        let public_key = VerifyingKey::from(&private_key);
let encoded_point = public_key.to_encoded_point(false);
let pubkey_bytes = encoded_point.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(pubkey_bytes);
        let address = hex::encode(hasher.finalize());

        Wallet {
            private_key,
            public_key,
            address,
        }
    }

    pub fn sign(&self, message: &str) -> Signature {
        self.private_key.sign(message.as_bytes())
    }

    pub fn verify(&self, message: &str, signature: &Signature) -> bool {
        self.public_key.verify(message.as_bytes(), signature).is_ok()
    }
}
