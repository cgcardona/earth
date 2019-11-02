use crate::{mainnet_seeders, testnet_seeders};
use clap;
use network::Network;
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

// pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }

#[derive(Debug)]
pub struct Configuration {
    pub network: Network,
    pub data_dir: Option<String>,
    port: u16,
    db_cache: usize,
    user_agent: String,
    quiet: bool,
    seeders: Vec<String>,
}

/// parse command line input
pub fn parse_input(matches: &clap::ArgMatches) -> Result<Configuration, String> {
    let is_testnet: bool = match matches.is_present("testnet") {
        true => true,
        false => false,
    };

    let is_regtest: bool = match matches.is_present("regtest") {
        true => true,
        false => false,
    };

    let mut network: Network = Network::Regtest;
    if is_regtest == false && is_testnet == false {
        network = Network::Mainnet;
    } else if is_regtest == true && is_testnet == true {
        return Err("Must choose mainnet, testnet or regtest".into());
    } else if is_regtest == false && is_testnet == true {
        network = Network::Testnet;
    } else if is_regtest == true && is_testnet == false {
        network = Network::Regtest;
    }

    let port: u16 = match matches.value_of("port") {
        Some(s) => s.parse().map_err(|_| "port is invalid".to_owned())?,
        None => network.port(),
    };

    let db_cache: usize = match matches.value_of("db-cache") {
        Some(s) => s.parse().map_err(|_| "db-cache is invalid".to_owned())?,
        None => 512,
    };

    let user_agent: String = String::from("/EARTH:0.0.1/");

    let quiet: bool = matches.is_present("quiet");

    let data_dir: Option<String> = match matches.value_of("data-dir") {
        Some(s) => Some(s.parse().map_err(|_| "Error".to_owned())?),
        None => None,
    };

    let seeders: Vec<String> = match matches.value_of("seednode") {
        Some(s) => vec![s.parse().map_err(|_| "Error".to_owned())?],
        None => match network {
            Network::Mainnet => mainnet_seeders(),
            Network::Testnet | Network::Regtest => testnet_seeders(),
        },
    };

    Ok(Configuration {
        network: network,
        port: port,
        db_cache: db_cache,
        user_agent: user_agent,
        quiet: quiet,
        data_dir: data_dir,
        seeders: seeders,
    })
}
