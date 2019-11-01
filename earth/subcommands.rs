use crate::Configuration;
use std::{fs, path::PathBuf};
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
    match configuration.data_dir {
        Some(ref data_dir) => create_data_dir(&data_dir),
        None => create_data_dir("data-dir"),
    };
    println!("{:#?}", &configuration);
}

/// create data_dir if it doesn't exist
fn create_data_dir(data_dir: &str) {
    fs::create_dir_all(data_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
