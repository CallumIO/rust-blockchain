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

    pub fn add_pending_transaction(&mut self, source: String, destination: String, data: String) {
        self.pending_transactions
            .push(Transaction::new(source, destination, data));
    }

    pub fn add_block(&mut self) {
        let prev_block = self.chain.last().unwrap().to_owned();
        let block = Block::next_block(prev_block, self.pending_transactions.to_owned());
        self.chain.push(block);
    }

    pub fn verify(&self) -> bool {
        let mut i = 1;
        while i < self.chain.len() {
            let cur_block = self.chain[i].to_owned();
            let prev_block = self.chain[i - 1].to_owned();
            if (cur_block.prev_hash != prev_block.hash)
                || (cur_block.hash != cur_block.hash_block())
            {
                return false;
            }

            i += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_new_block() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            "One".to_string(),
            "Another".to_string(),
            "30000".to_string(),
        );
        bc.add_block();
        let last_block = bc.chain.last().unwrap().to_owned();
        assert_eq!(last_block.block_id, 1i64);
        assert_eq!(last_block.data[0].source, "One");
        assert_eq!(last_block.data[0].destination, "Another");
        assert_eq!(last_block.data[0].data, "30000");
    }

    #[test]
    fn add_pending_transaction_to_array() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            "One".to_string(),
            "Another".to_string(),
            "30000".to_string(),
        );
        assert_eq!(bc.pending_transactions[0].source, "One");
        assert_eq!(bc.pending_transactions[0].destination, "Another");
        assert_eq!(bc.pending_transactions[0].data, "30000");
    }

    #[test]
    fn pending_transactions_removed_on_new_block() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            "One".to_string(),
            "Another".to_string(),
            "30000".to_string(),
        );
        bc.add_block();
        assert!(bc.pending_transactions.is_empty());
    }
}
