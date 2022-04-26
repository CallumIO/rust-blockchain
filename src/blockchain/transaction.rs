use chrono::prelude::*;
use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: i64,
    pub source: String,
    pub destination: String,
    pub block_id: i64
    pub data: String,
}

impl Transaction {
    pub fn new(source: String, destination: String, block_id: i64) -> Transaction {
        return Transaction {
            timestamp: Utc::now().timestamp(),
            source: source,
            destination: destination,
            block_id: block_id,
            data: data,
        }
    }
}
