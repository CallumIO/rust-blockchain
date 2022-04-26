mod block;
mod transaction;
use block::Block;
use transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        return Blockchain {
            chain: vec![Block::genesis()],
            pending_transactions: vec![],
        };
    }
}
