use crate::{mainnet_seeders, testnet_seeders};
use clap;
use core::option::Option;
use network::Network;
use p2p::{ConsensusParams, IP};
use rpc::{HttpConfig, RPC};
use services::Services;
use std::{
    net,
    net::{IpAddr, SocketAddr},
};
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
    pub services: Services,
    pub data_dir: Option<String>,
    pub consensus: ConsensusParams,
    pub port: u16,
    pub db_cache: usize,
    pub ua: String,
    pub quiet: bool,
    pub seeders: Vec<String>,
    pub host: IpAddr,
    pub inbound_connections: u32,
    pub outbound_connections: u32,
    pub threads: usize,
    pub connect: Option<net::SocketAddr>,
    pub ip: IP,
    pub rpc_config: HttpConfig,
    pub block_notify_command: Option<String>,
    pub verification_params: (),
    pub db: (),
}

impl Configuration {
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

        let (outbound_connections, inbound_connections, threads): (u32, u32, usize) = match network
        {
            Network::Mainnet | Network::Testnet => (10, 10, 4),
            Network::Regtest => (0, 1, 1),
        };

        let only_net: p2p::IP = IP::IPV4;

        let port: u16 = match matches.value_of("port") {
            Some(s) => s
                .parse()
                .map_err(|_| String::from("port is invalid"))
                .unwrap(),
            None => network.port(),
        };

        let db_cache: usize = match matches.value_of("db-cache") {
            Some(s) => s
                .parse()
                .map_err(|_| String::from("db-cache is invalid"))
                .unwrap(),
            None => 512,
        };

        let ua: String = "/EARTH:0.0.1/".into();

        let quiet: bool = matches.is_present("quiet");

        let data_dir: Option<String> = match matches.value_of("data-dir") {
            Some(s) => Some(s.parse().map_err(|_| String::from("Error")).unwrap()),
            None => None,
        };

        let seeders: Vec<String> = match matches.value_of("seednode") {
            Some(s) => vec![s.parse().map_err(|_| String::from("Error")).unwrap()],
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
                    .map_err(|_| String::from("Invalid connect")),
                Ok(a) => Ok(a),
            }?),
            None => None,
        };

        let consensus: ConsensusParams = ConsensusParams::return_params(network);

        let services: Services = Services::default().with_network(true);

        let host: IpAddr = match matches.value_of("host") {
            Some(s) => s
                .parse::<net::IpAddr>()
                .map_err(|_| String::from("Invalid host"))?,
            None => match only_net {
                IP::IPV6 => "::".parse().unwrap(),
                _ => "0.0.0.0".parse().unwrap(),
            },
        };

        let rpc_config: HttpConfig = RPC::parse_rpc_config(network, matches).unwrap();
        let block_notify_command = Some("foo".into());
        let verification_params = ();
        let db = ();

        Ok(Configuration {
            network: network,
            port: port,
            db_cache: db_cache,
            ua: ua,
            quiet: quiet,
            consensus: consensus,
            data_dir: data_dir,
            seeders: seeders,
            inbound_connections: inbound_connections,
            threads: threads,
            outbound_connections: outbound_connections,
            connect: connect,
            ip: only_net,
            services: services,
            host: host,
            rpc_config: rpc_config,
            block_notify_command: block_notify_command,
            verification_params: verification_params,
            db: db,
        })
    }
}
