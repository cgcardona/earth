use chain::{Block, BlockHeader, Transaction};
use std::time::SystemTime;

fn main() {
    let version: u32 = 1;
    let bits: u8 = 1;
    let nonce: u32 = 1;
    let merkle_root_hash: u8 = 1;
    let prev_header_hash: u8 = 1;
    let time: SystemTime = SystemTime::now();

    let block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );
    let transcation: Transaction = Transaction::new();
    let block: Block = Block::new(block_header, vec![transcation]);
    println!("Hello, {:#?}!", block.block_header().time());
    // println!("----------");
    // println!("Header: {:#?}!", b.block_header());
    // println!("----------");
    // println!("Transactions: {:#?}!", b.transactions());
}
