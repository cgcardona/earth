use crate::{mainnet_seeders, testnet_seeders};
use clap;
use core::option::Option;
use network::Network;
use p2p::{ConsensusParams, Deployment, IP};
use std::net;
use std::net::SocketAddr;
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
    pub consensus: ConsensusParams,
    pub port: u16,
    pub db_cache: usize,
    pub user_agent: String,
    pub quiet: bool,
    pub seeders: Vec<String>,
    pub inbound_connections: u32,
    pub outbound_connections: u32,
    pub threads: u32,
    pub connect: Option<net::SocketAddr>,
    pub internet_protocol: IP,
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

    let (inbound_connections, outbound_connections, threads): (u32, u32, u32) = match network {
        Network::Mainnet | Network::Testnet => (10, 10, 4),
        Network::Regtest => (1, 0, 1),
    };

    let only_net: p2p::IP = IP::IPV4;

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

    let connect: Option<SocketAddr> = match matches.value_of("connect") {
        Some(s) => Some(match s.parse::<net::SocketAddr>() {
            Err(_) => s
                .parse::<net::IpAddr>()
                .map(|ip| net::SocketAddr::new(ip, network.port()))
                .map_err(|_| "Invalid connect".to_owned()),
            Ok(a) => Ok(a),
        }?),
        None => None,
    };

    let consensus = match network {
        Network::Mainnet => ConsensusParams {
            network: network,
            bip16_time: 1333238400,                 // Apr 1 2012
            bip34_height: 227931, // 000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8
            bip65_height: 388381, // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
            bip66_height: 363725, // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
            rule_change_activation_threshold: 1916, // 95%
            miner_confirmation_window: 2016,
            csv_deployment: Some(Deployment {
                name: "csv",
                bit: 0,
                start_time: 1462060800,
                timeout: 1493596800,
                activation: Some(419328),
            }),
        },
        Network::Testnet => ConsensusParams {
            network: network,
            bip16_time: 1333238400,                 // Apr 1 2012
            bip34_height: 21111, // 0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8
            bip65_height: 581885, // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
            bip66_height: 330776, // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
            rule_change_activation_threshold: 1512, // 75%
            miner_confirmation_window: 2016,
            csv_deployment: Some(Deployment {
                name: "csv",
                bit: 0,
                start_time: 1456790400,
                timeout: 1493596800,
                activation: Some(770112),
            }),
        },
        Network::Regtest => ConsensusParams {
            network: network,
            bip16_time: 1333238400,  // Apr 1 2012
            bip34_height: 100000000, // not activated on regtest
            bip65_height: 1351,
            bip66_height: 1251,                    // used only in rpc tests
            rule_change_activation_threshold: 108, // 75%
            miner_confirmation_window: 144,
            csv_deployment: Some(Deployment {
                name: "csv",
                bit: 0,
                start_time: 0,
                timeout: 0,
                activation: Some(0),
            }),
        },
    };

    Ok(Configuration {
        network: network,
        port: port,
        db_cache: db_cache,
        user_agent: user_agent,
        quiet: quiet,
        consensus: consensus,
        data_dir: data_dir,
        seeders: seeders,
        inbound_connections: inbound_connections,
        threads: threads,
        outbound_connections: outbound_connections,
        connect: connect,
        internet_protocol: only_net,
    })
}
