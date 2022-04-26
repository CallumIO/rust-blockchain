use crate::blockchain::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};

pub struct Block {
    timestamp: i64,
    hash: String,
    block_id: i64,
    prev_hash: String,
    data: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: String, data: Vec<Transaction>) -> Block {
        return Block {};
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
