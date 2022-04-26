use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_transaction() {
        let my_transaction = Transaction::new(
            "One".to_string(),
            "Another".to_string(),
            "30000".to_string(),
        );
        assert_eq!(my_transaction.timestamp, Utc::now().timestamp()); //TODO: may fail if slow to execute?
        assert_eq!(my_transaction.source, "One");
        assert_eq!(my_transaction.destination, "Another");
        assert_eq!(my_transaction.data, "30000");
    }
}
