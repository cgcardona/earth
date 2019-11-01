use crate::Configuration;
use std::{fs::create_dir_all, path::PathBuf};
// use network::Network;

/// imports blockchain data
pub fn import(configuration: &Configuration, matches: &clap::ArgMatches) {
    let genesis: String = configuration.network.genesis();
    let import_path: &str = matches.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", matches, genesis, import_path);
}

/// Rollback the database to block hash or number
pub fn rollback(configuration: &Configuration, matches: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", configuration, matches);
}

/// start EARTH client with command line arguments
pub fn start(configuration: Configuration) {
    // create db directory
    match configuration.data_dir {
        Some(ref data_dir) => create_data_dir(&data_dir, "db"),
        None => create_data_dir("data-dir", "db"),
    };

    // create p2p directory
    match configuration.data_dir {
        Some(ref data_dir) => create_p2p_dir(&data_dir, "p2p"),
        None => create_p2p_dir("data-dir", "p2p"),
    };
}

/// create data_dir if it doesn't exist
fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [data_dir, sub].iter().collect();
    create_dir_all(&p).expect("Failed to get app dir");
    p
}

/// create p2p directory if it doesn't exist
fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [p2p, sub].iter().collect();
    create_dir_all(&p).expect("Failed to get app dir");
    p
}
