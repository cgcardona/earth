// use crate::seeders::{mainnet_seeders, testnet_seeders};
use crate::Configuration;
// use blockchain::Block;
// use database::*;
// use mock_data::block_mock_data;
use std::{fs, path::PathBuf};

/// imports blockchain data
pub fn import(c: &Configuration, m: &clap::ArgMatches) {
    let g: &str = c.network.genesis();
    let i: &str = m.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", m, g, i);
}

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, m: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", c, m);
}

/// start EARTH client with command line arguments
pub fn start(c: Configuration) {
    start_db(&c);
    start_p2p(&c);
}

/// Start database setup
fn start_db(c: &Configuration) {
    // create db directory
    match c.data_dir {
        Some(ref data_dir) => create_data_dir(&data_dir, "db"),
        None => create_data_dir("data-dir", "db"),
    };
}

/// Start p2p connections
fn start_p2p(c: &Configuration) {
    // create p2p directory
    match c.data_dir {
        Some(ref data_dir) => create_p2p_dir(&data_dir, "p2p"),
        None => create_p2p_dir("data-dir", "p2p"),
    };
}

/// create data_dir if it doesn't exist
fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [data_dir, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get app dir");

    p
}

/// create p2p directory if it doesn't exist
fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [p2p, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get p2p dir");

    p
}
