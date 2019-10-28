#[macro_use]
extern crate clap;
use chain::{Block, BlockHeader, Transaction, Vin, Vout};
pub use clap::App;
use commands::start;
use config::{calculate_hash, parse};
use std::time::SystemTime;

// mod commands;
mod config;
// mod rpc;
// mod rpc_apis;
// mod seednodes;
mod util;

pub const USER_AGENT: &'static str = "earth";
pub const REGTEST_USER_AGENT: &'static str = "/EARTH:0.0.1/";

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches: clap::ArgMatches<'_> = clap::App::from_yaml(yml).get_matches();
    let cfg = parse(&matches);
    println!("MATCHES: {:#?}", cfg);

    start();
    let version: u32 = 1;
    let bits: u32 = 1;
    let nonce: u32 = 1;
    let merkle_root_hash: u64 = 1;
    let prev_header_hash: u64 = 1;
    let time: SystemTime = SystemTime::now();

    let prev_block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );

    let prev_header_hash: u64 = calculate_hash(&prev_block_header);
    let merkle_root_hash: u64 = calculate_hash(&prev_block_header);
    let block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );

    let txid: String =
        String::from("fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33");
    let version: u32 = 1;
    let locktime: u32 = 1;
    let vin: Vec<Vin> = vec![];
    let vout: Vec<Vout> = vec![];
    let blockhash: String =
        String::from("00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09");
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
    // println!("{:#?}!", block);
    // println!("----------");
    // println!("{:#?}!", block.block_header());
    // println!("----------");
    // println!("{:#?}!", block.transactions());
}
