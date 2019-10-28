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
    pub quiet: bool,
    // pub inbound_connections: u32,
    // pub outbound_connections: u32,
    // pub p2p_threads: usize,
    pub db_cache: usize,
    pub data_dir: Option<String>,
    // pub user_agent: String,
    // pub internet_protocol: InternetProtocol,
    // pub rpc_config: RpcHttpConfig,
    // pub block_notify_command: Option<String>,
    // pub verification_params: VerificationParameters,
    // pub db: storage::SharedStore,
}

pub const DEFAULT_DB_CACHE: usize = 512;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
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
        // port: port,
        quiet: quiet,
        db_cache: db_cache,
        data_dir: data_dir,
        // network: network,
        // host: host,
        // user_agent: user_agent,
    })
}
