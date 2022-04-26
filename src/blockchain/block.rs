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

impl Block {}
