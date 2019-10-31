use crate::Configuration;
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
pub fn start(configuration: &Configuration) {
    println!("START: {:#?}", configuration);
}
