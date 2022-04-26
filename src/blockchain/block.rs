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
            prev_hash.to_owned(),
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
        return Block::new(
            "0".to_string(),
            0i64,
            vec![Transaction::new(
                "The Beginning".to_string(),
                "The End".to_string(),
                "{}".to_string(),
            )],
        );
    }
    pub fn next_block(last_block: Block, data: Vec<Transaction>) -> Block {
        return Block::new(last_block.hash, last_block.block_id + 1, data);
    }
    pub fn block_details(&self) -> String {
        let mut transactions: Vec<String> = vec![];
        for transaction in self.data.iter() {
            transactions.push(transaction.timestamp.to_owned().to_string());
            transactions.push(transaction.source.to_owned());
            transactions.push(transaction.destination.to_owned());
            transactions.push(transaction.data.to_owned());
        }
        return transactions.join("\n");
    }
}
fn byte_array_to_hex(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    return strs.join(" ");
}
