mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    println!("{}", bc.chain[0].block_details());
}
