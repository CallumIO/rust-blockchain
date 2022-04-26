mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_pending_transaction(
        "callum".to_string(),
        "not callum".to_string(),
        "3000".to_string(),
    );
    bc.add_pending_transaction(
        "not callum".to_string(),
        "callum".to_string(),
        "1000".to_string(),
    );
    bc.add_block();
    println!("{}", bc.chain[0].block_details());
    println!("{}", bc.chain[1].block_details());
    println!("{:?}", bc.verify());
}
