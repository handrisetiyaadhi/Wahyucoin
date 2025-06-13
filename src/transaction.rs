use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub signature: String,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64, signature: String, timestamp: i64) -> Self {
        Self {
            from,
            to,
            amount,
            signature,
            timestamp,
        }
    }
}
