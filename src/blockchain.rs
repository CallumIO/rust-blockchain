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

    pub fn add_block(&mut self) {
        let prev_block = self.chain.last().unwrap().to_owned();
        let block = Block::next_block(prev_block, self.pending_transactions.to_owned());
        self.chain.push(block);
    }
}
