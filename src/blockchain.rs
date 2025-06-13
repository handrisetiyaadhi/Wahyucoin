use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::genesis()],
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let data = serde_json::to_string(&transactions).unwrap();
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::new(last_block, data, self.difficulty);
        self.blocks.push(new_block);
    }
}
