mod block;
mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use transaction::Transaction;
use wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("Miner address: {}", wallet.address);

    let mut chain = Blockchain::new();

let mut tx1 = Transaction::new(
    "satoshi".to_string(),
    wallet.address.clone(),
    50.0,
    "".to_string(),         // signature kosong, nanti diisi
    chrono::Utc::now().timestamp()
);
let mut tx2 = Transaction::new(
    wallet.address.clone(),           // from: String
    "bob".to_string(),                // to: String
    20.0,                             // amount
    "".to_string(),                   // signature kosong dulu
    chrono::Utc::now().timestamp()    // timestamp
);

    // Tanda tangan transaksi kedua (dari wallet sendiri)
let sig = wallet.sign(&format!("{:?}", tx2));
    chain.add_block(vec![tx1, tx2]);

    for block in chain.blocks.iter() {
        println!("{:#?}", block);
    }
}
