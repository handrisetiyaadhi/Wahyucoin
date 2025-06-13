use chrono::prelude::*; use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f32,
    pub signature: String,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f32, signature: String, timestamp: i64) -> Self {
        Self {
            from,
            to,
            amount,
            signature,
            timestamp,
        }
    }
}
