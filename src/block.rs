use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(previous: &Block, data: String, difficulty: usize) -> Self {
        let index = previous.index + 1;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let previous_hash = previous.hash.clone();

        let (nonce, hash) = Block::mine(index, timestamp, &data, &previous_hash, difficulty);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn genesis() -> Self {
        Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_string());
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn mine(index: u64, timestamp: u128, data: &str, previous_hash: &str, difficulty: usize) -> (u64, String) {
        let prefix = "0".repeat(difficulty);
        let mut nonce = 0;

        loop {
            let hash = Block::calculate_hash(index, timestamp, data, previous_hash, nonce);
            if hash.starts_with(&prefix) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
