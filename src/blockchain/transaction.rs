use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: i64,
    pub source: String,
    pub destination: String,
    pub data: String,
}

impl Transaction {
    pub fn new(source: String, destination: String, data: String) -> Transaction {
        return Transaction {
            timestamp: Utc::now().timestamp(),
            source: source,
            destination: destination,
            data: data,
        };
    }
}
