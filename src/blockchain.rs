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
        let prev_block = self.chain.last().unwrap();
        let block = Block::next_block(prev_block, &self.pending_transactions);
        self.pending_transactions = vec![];
        self.chain.push(block);
    }

    pub fn verify(&self) -> bool {
        let mut i = 1;
        while i < self.chain.len() {
            let cur_block = &self.chain[i];
            let prev_block = &self.chain[i - 1];
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
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        let last_block = bc.chain.last().unwrap();
        assert_eq!(last_block.block_id, 1i64);
        assert_eq!(last_block.data[0].source, "One");
        assert_eq!(last_block.data[0].destination, "Another");
        assert_eq!(last_block.data[0].data, "30000");
    }

    #[test]
    fn add_pending_transaction_to_array() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        assert_eq!(bc.pending_transactions[0].source, "One");
        assert_eq!(bc.pending_transactions[0].destination, "Another");
        assert_eq!(bc.pending_transactions[0].data, "30000");
    }

    #[test]
    fn pending_transactions_removed_on_new_block() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        assert!(bc.pending_transactions.is_empty());
    }

    #[test]
    fn chain_verification_unchanged() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        bc.add_pending_transaction(
            String::from("Another"),
            String::from("One"),
            String::from("20000"),
        );
        bc.add_block();
        assert!(bc.verify());
    }

    #[test]
    fn chain_verification_changed_data() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        bc.add_pending_transaction(
            String::from("Another"),
            String::from("One"),
            String::from("20000"),
        );
        bc.add_block();
        bc.chain[1].data = vec![Transaction::new(
            String::from("One"),
            String::from("Another"),
            String::from("20000"),
        )];
        assert!(!bc.verify());
    }

    #[test]
    fn chain_verification_changed_prev_hash() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        bc.add_pending_transaction(
            String::from("Another"),
            String::from("One"),
            String::from("20000"),
        );
        bc.add_block();
        bc.chain[1].prev_hash = "changed prev_hash".to_string();
        assert!(!bc.verify());
    }

    #[test]
    fn chain_verification_changed_hash() {
        let mut bc = Blockchain::new();
        bc.add_pending_transaction(
            String::from("One"),
            String::from("Another"),
            String::from("30000"),
        );
        bc.add_block();
        bc.add_pending_transaction(
            String::from("Another"),
            String::from("One"),
            String::from("20000"),
        );
        bc.add_block();
        bc.chain[1].hash = "changed hash".to_string();
        assert!(!bc.verify());
    }
}
