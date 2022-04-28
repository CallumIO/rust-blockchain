use crate::blockchain::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::convert::TryInto;

#[derive(Clone)]
pub struct Block {
    pub timestamp: i64,
    pub hash: String,
    pub block_id: i64,
    pub prev_hash: String,
    pub data: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_hash: &String, block_id: i64, data: Vec<Transaction>) -> Block {
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
            timestamp,
            hash,
            block_id,
            prev_hash: prev_hash.to_owned(),
            data,
        };
    }
    pub fn genesis() -> Block {
        return Block::new(
            &"0".to_string(),
            0i64,
            vec![Transaction::new(
                "The Beginning".to_string(),
                "The End".to_string(),
                "{}".to_string(),
            )],
        );
    }

    pub fn hash_block(&self) -> String {
        let mut sha = Sha256::new();
        let to_hash = [
            self.timestamp.to_string(),
            self.prev_hash.to_owned(),
            byte_array_to_hex(bincode::serialize(&self.data).unwrap()),
        ];
        sha.update(to_hash.join(""));
        let hash_bytes = sha.finalize();
        let hash = byte_array_to_hex(
            hash_bytes
                .as_slice()
                .try_into()
                .expect("Error hashing block"),
        );
        return hash;
    }

    pub fn next_block(last_block: &Block, data: &Vec<Transaction>) -> Block {
        return Block::new(&last_block.hash, last_block.block_id + 1, data.to_owned());
    }

    pub fn block_details(&self) -> String {
        let mut transactions = vec![];
        for transaction in self.data.iter() {
            transactions.push(format!(
                "{}\n{}\n{}\n{}\n",
                &transaction.timestamp.to_string(),
                &transaction.source,
                &transaction.destination,
                &transaction.data
            ));
        }
        return format!(
            "Begin Block {}\nWith Hash: {}\nPrevious Hash: {}\n\n{}End Block {}\n",
            self.block_id,
            self.hash,
            self.prev_hash,
            transactions.join("\n"),
            self.block_id,
        );
    }
}

fn byte_array_to_hex(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    return strs.join("").to_lowercase();
}

#[cfg(test)]
mod test {
    use super::*;

    //TODO: Hash and transaction data cannot be tested due to using a timestamp. Refactoring necessary to enable testing
    #[test]
    fn create_new_block() {
        let block = Block::new(
            &"Previous Hash".to_string(),
            99,
            vec![
                Transaction::new(
                    "One".to_string(),
                    "Another".to_string(),
                    "30000".to_string(),
                ),
                Transaction::new(
                    "Another".to_string(),
                    "One".to_string(),
                    "20000".to_string(),
                ),
            ],
        );
        assert_eq!(block.prev_hash, "Previous Hash");
        assert_eq!(block.block_id, 99);
    }

    #[test]
    fn rehash_block() {
        let block = Block::genesis();
        assert_eq!(block.hash_block(), block.hash);
    }

    #[test]
    fn rehash_changed_block() {
        let mut block = Block::genesis();
        block.prev_hash = "Not the same as before".to_string();
        assert_ne!(block.hash_block(), block.hash);
    }

    #[test]
    fn create_next_block() {
        let genesis = Block::genesis();
        let gen_hash = &genesis.hash;
        let block = Block::next_block(
            &genesis,
            &vec![
                Transaction::new(
                    "One".to_string(),
                    "Another".to_string(),
                    "30000".to_string(),
                ),
                Transaction::new(
                    "Another".to_string(),
                    "One".to_string(),
                    "20000".to_string(),
                ),
            ],
        );
        assert_eq!(&block.prev_hash, gen_hash);
        assert_eq!(block.block_id, 1);
    }

    #[test]
    fn bytes_convert_to_hex_string() {
        assert_eq!(
            byte_array_to_hex(vec![84, 101, 115, 116, 32, 86, 97, 108, 117, 101, 10]),
            "546573742056616c75650a"
        );
    }
}
