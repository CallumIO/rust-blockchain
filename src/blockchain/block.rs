use crate::blockchain::transaction::Transaction;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

pub struct Block {
    timestamp: i64,
    hash: String,
    block_id: i64,
    prev_hash: String,
    data: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: String, block_id: i64, data: Vec<Transaction>) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut sha = Sha256::new();
        let to_hash = [
            timestamp.to_string(),
            prev_hash,
            byte_array_to_hex(bincode::serialize(&data).unwrap()),
        ];
        sha.update(to_hash.join(""));
        let hash_bytes = sha.finalize();
        let hash = byte_array_to_hex(
            hash_bytes
                .as_slice()
                .try_into()
                .expect("Error hashing block"),
        );
        return Block {
            timestamp: timestamp,
            hash: hash,
            block_id: block_id,
            prev_hash: prev_hash,
            data: data,
        };
    }
    pub fn genesis() -> Block {
        return Block {};
    }
    pub fn next_block(last_block: Block, data: Vec<Transaction>) -> Block {
        return Block {};
    }
    pub fn block_details(&self) -> &str {
        return "";
    }
}
pub fn byte_array_to_hex(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    return strs.join(" ");
}
