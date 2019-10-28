use chain::{Block, BlockHeader, Transaction, Vin, Vout};
use std::time::SystemTime;

fn main() {
    let version: u32 = 1;
    let bits: u32 = 1;
    let nonce: u32 = 1;
    let merkle_root_hash: u32 = 1;
    let prev_header_hash: u32 = 1;
    let time: SystemTime = SystemTime::now();
    let block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );

    let txid: String = String::from("hello world");
    let version: u32 = 1;
    let locktime: u32 = 1;
    let vin: Vec<Vin> = vec![];
    let vout: Vec<Vout> = vec![];
    let blockhash: String = String::from("hello world");
    let blockheight: u32 = 1;
    let confirmations: u32 = 1;
    let time: SystemTime = SystemTime::now();
    let blocktime: SystemTime = SystemTime::now();
    let isCoinBase: bool = false;
    let valueOut: u32 = 1;
    let size: u32 = 1;

    let transcation: Transaction = Transaction::new(
        txid,
        version,
        locktime,
        vin,
        vout,
        blockhash,
        blockheight,
        confirmations,
        time,
        blocktime,
        isCoinBase,
        valueOut,
        size,
    );
    let block: Block = Block::new(block_header, vec![transcation]);
    println!("Hello, {:#?}!", block.block_header().time());
    // println!("----------");
    // println!("Header: {:#?}!", b.block_header());
    // println!("----------");
    // println!("Transactions: {:#?}!", b.transactions());
}
