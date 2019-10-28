// use consensus::Network;
// use p2p::InternetProtocol;
use serde_derive::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net;

// TODO : Call these from src/main.rs
pub const USER_AGENT: &'static str = "earth";
pub const REGTEST_USER_AGENT: &'static str = "/EARTH:0.0.1/";

#[derive(Deserialize, Debug)]
pub struct Config {
    // pub network: Network,
    // pub consensus: ConsensusParams,
    // pub services: Services,
    // pub port: u16,
    // pub connect: Option<net::SocketAddr>,
    // pub host: net::IpAddr,
    // pub seednodes: Vec<String>,
    // pub inbound_connections: u32,
    // pub outbound_connections: u32,
    // pub p2p_threads: usize,
    // pub user_agent: String,
    // pub internet_protocol: InternetProtocol,
    // pub rpc_config: RpcHttpConfig,
    // pub block_notify_command: Option<String>,
    // pub verification_params: VerificationParameters,
    // pub db: storage::SharedStore,
    pub db_cache: usize,
    pub consensus: String,
    pub network: String,
    pub services: String,
    pub port: String,
    pub connect: String,
    pub host: String,
    pub seednodes: String,
    pub quiet: bool,
    pub inbound_connections: String,
    pub outbound_connections: String,
    pub p2p_threads: String,
    pub data_dir: Option<String>,
    pub user_agent: String,
    pub internet_protocol: String,
    pub rpc_config: String,
    pub block_notify_command: String,
    pub verification_params: String,
    pub db: String,
}

pub const DEFAULT_DB_CACHE: usize = 512;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
    let network = match matches.value_of("network") {
        Some(s) => String::from("network"),
        None => String::from("network"),
    };
    let consensus: String = String::from("consensus");
    let services: String = String::from("services");
    let port: String = String::from("port");
    let connect: String = String::from("connect");
    let host: String = String::from("host");
    let seednodes: String = String::from("seednodes");
    let inbound_connections: String = String::from("inbound_connections");
    let outbound_connections: String = String::from("outbound_connections");
    let p2p_threads: String = String::from("p2p_threads");
    let user_agent: String = String::from("user_agent");
    let internet_protocol: String = String::from("internet_protocol");
    let rpc_config: String = String::from("rpc_config");
    let block_notify_command: String = String::from("block_notify_command");
    let verification_params: String = String::from("verification_params");
    let db: String = String::from("db");

    let db_cache = match matches.value_of("db-cache") {
        Some(s) => s
            .parse()
            .map_err(|_| "Invalid cache size - should be number in MB".to_owned())?,
        None => DEFAULT_DB_CACHE,
    };

    let data_dir = match matches.value_of("data-dir") {
        Some(s) => Some(s.parse().map_err(|_| "Invalid data-dir".to_owned())?),
        None => None,
    };

    let quiet = matches.is_present("quiet");
    // let network = match (matches.is_present("testnet"), matches.is_present("regtest")) {
    //     (true, false) => Network::Testnet,
    //     (false, true) => Network::Regtest,
    //     (false, false) => Network::Testnet,
    //     (true, true) => return Err("Only one testnet option can be used".into()),
    // };

    // let port = match matches.value_of("port") {
    //     Some(port) => port.parse().map_err(|_| "Invalid port".to_owned())?,
    //     None => network.port(),
    // };

    // let only_net = match matches.value_of("only-net") {
    //     Some(s) => s.parse()?,
    //     None => InternetProtocol::default(),
    // };

    // let host = match matches.value_of("host") {
    //     Some(s) => s
    //         .parse::<net::IpAddr>()
    //         .map_err(|_| "Invalid host".to_owned())?,
    //     None => match only_net {
    //         InternetProtocol::IpV6 => "::".parse().unwrap(),
    //         _ => "0.0.0.0".parse().unwrap(),
    //     },
    // };
    // let user_agent = match network {
    //     Network::Testnet | Network::Mainnet | Network::Unitest | Network::Other(_) => {
    //         format!("{}", USER_AGENT)
    //     }
    //     Network::Regtest => REGTEST_USER_AGENT.into(),
    // };

    Ok(Config {
        port: port,
        quiet: quiet,
        db_cache: db_cache,
        data_dir: data_dir,
        network: network,
        consensus: consensus,
        services: services,
        connect: connect,
        host: host,
        seednodes: seednodes,
        inbound_connections: inbound_connections,
        outbound_connections: outbound_connections,
        p2p_threads: p2p_threads,
        user_agent: user_agent,
        internet_protocol: internet_protocol,
        rpc_config: rpc_config,
        block_notify_command: block_notify_command,
        verification_params: verification_params,
        db: db,
    })
}
