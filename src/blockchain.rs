use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::genesis();
        Blockchain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block = self.blocks.last().unwrap();
        let data = serde_json::to_string(&transactions).unwrap(); // convert tx to string
        let new_block = Block::new(last_block, data);
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
            );

            if current.hash != recalculated_hash {
                return false;
            }
        }

        true
    }
}
