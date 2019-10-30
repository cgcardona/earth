use clap;
use network::Network;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
pub struct Config {
    network: Network,
}

pub fn parse_input(matches: &clap::ArgMatches) -> Result<Config, String> {
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
    } else if is_regtest == false && is_testnet != true {
        network = Network::Testnet;
    } else if is_regtest == true && is_testnet != false {
        network = Network::Regtest;
    }

    // let network: Network = match (matches.is_present("testnet"), matches.is_present("regtest")) {
    //     (false, false) => Network::Mainnet,
    //     (true, false) => Network::Testnet,
    //     (false, true) => Network::Regtest,
    //     (true, true) => return Err("Only one testnet option can be used".into()),
    // };

    Ok(Config { network: network })
}
