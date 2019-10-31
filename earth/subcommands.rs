use crate::Configuration;
use network::Network;

/// imports blockchain data
pub fn import(configuration: &Configuration, matches: &clap::ArgMatches) {
    let genesis: String = configuration.network.genesis();
    let import_path: &str = matches.value_of("PATH").unwrap();
    // println!("IMPORT: {:#?}, {}, {}", matches, genesis, import_path);
}
