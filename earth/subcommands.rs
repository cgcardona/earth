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
        // TODO : prevent from rm -rf
        Some(_) => {
            // if --data-dir flag was passed in then create that directory
            fs::create_dir_all(&configuration.data_dir.unwrap()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
        None => {
            // else create data-dir/ directory
            fs::create_dir_all("data-dir").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
    };
    // println!("{:#?}", &configuration);
}
