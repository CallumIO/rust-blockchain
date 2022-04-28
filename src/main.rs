mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_pending_transaction(
        String::from("callum"),
        String::from("not callum"),
        String::from("3000"),
    );
    bc.add_pending_transaction(
        String::from("not callum"),
        String::from("callum"),
        String::from("1000"),
    );
    bc.add_block();
    println!("{}", bc.chain[0].block_details());
    println!("{}", bc.chain[1].block_details());
    println!("{:?}", bc.verify());
}
