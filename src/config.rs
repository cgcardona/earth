use clap;
// use message::Services;
// use network::{BitcoinCashConsensusParams, ConsensusFork, ConsensusParams, Network};
use p2p::InternetProtocol;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use primitives::hash::H256;
// use rpc::HttpConfiguration as RpcHttpConfig;
// use rpc_apis::ApiSet;
// use seednodes::{
//     bitcoin_cash_seednodes, bitcoin_cash_testnet_seednodes, mainnet_seednodes, testnet_seednodes,
// };
use std::net;
// use storage;
// use sync::VerificationParameters;
// use util::open_db;
// use verification::VerificationLevel;
// use {REGTEST_USER_AGENT, USER_AGENT};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
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

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
    // let db_cache = match matches.value_of("db-cache") {
    //     Some(s) => s
    //         .parse()
    //         .map_err(|_| "Invalid cache size - should be number in MB".to_owned())?,
    //     None => DEFAULT_DB_CACHE,
    // };

    // let data_dir = match matches.value_of("data-dir") {
    //     Some(s) => Some(s.parse().map_err(|_| "Invalid data-dir".to_owned())?),
    //     None => None,
    // };

    // let db = open_db(&data_dir, db_cache);

    // let quiet = matches.is_present("quiet");
    // let network = match (matches.is_present("testnet"), matches.is_present("regtest")) {
    //     (true, false) => Network::Testnet,
    //     (false, true) => Network::Regtest,
    //     (false, false) => Network::Mainnet,
    //     (true, true) => return Err("Only one testnet option can be used".into()),
    // };

    // let consensus_fork = parse_consensus_fork(network, &db, &matches)?;
    // let consensus = ConsensusParams::new(network, consensus_fork);

    // let (in_connections, out_connections) = match network {
    //     Network::Testnet | Network::Mainnet | Network::Other(_) => (10, 10),
    //     Network::Regtest | Network::Unitest => (1, 0),
    // };

    // let p2p_threads = match network {
    //     Network::Testnet | Network::Mainnet | Network::Other(_) => 4,
    //     Network::Regtest | Network::Unitest => 1,
    // };

    // // to skip idiotic 30 seconds delay in test-scripts
    // let user_agent_suffix = match consensus.fork {
    //     ConsensusFork::BitcoinCore => "",
    //     ConsensusFork::BitcoinCash(_) => "/UAHF",
    // };
    // let user_agent = match network {
    //     Network::Testnet | Network::Mainnet | Network::Unitest | Network::Other(_) => {
    //         format!("{}{}", USER_AGENT, user_agent_suffix)
    //     }
    //     Network::Regtest => REGTEST_USER_AGENT.into(),
    // };

    // let port = match matches.value_of("port") {
    //     Some(port) => port.parse().map_err(|_| "Invalid port".to_owned())?,
    //     None => network.port(),
    // };

    // let connect = match matches.value_of("connect") {
    //     Some(s) => Some(match s.parse::<net::SocketAddr>() {
    //         Err(_) => s
    //             .parse::<net::IpAddr>()
    //             .map(|ip| net::SocketAddr::new(ip, network.port()))
    //             .map_err(|_| "Invalid connect".to_owned()),
    //         Ok(a) => Ok(a),
    //     }?),
    //     None => None,
    // };

    // let seednodes: Vec<String> = match matches.value_of("seednode") {
    //     Some(s) => vec![s.parse().map_err(|_| "Invalid seednode".to_owned())?],
    //     None => match (network, &consensus.fork) {
    //         (Network::Mainnet, &ConsensusFork::BitcoinCash(_)) => bitcoin_cash_seednodes()
    //             .into_iter()
    //             .map(Into::into)
    //             .collect(),
    //         (Network::Testnet, &ConsensusFork::BitcoinCash(_)) => bitcoin_cash_testnet_seednodes()
    //             .into_iter()
    //             .map(Into::into)
    //             .collect(),
    //         (Network::Mainnet, _) => mainnet_seednodes().into_iter().map(Into::into).collect(),
    //         (Network::Testnet, _) => testnet_seednodes().into_iter().map(Into::into).collect(),
    //         (Network::Other(_), _) | (Network::Regtest, _) | (Network::Unitest, _) => Vec::new(),
    //     },
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

    // let rpc_config = parse_rpc_config(network, matches)?;

    // let block_notify_command = match matches.value_of("blocknotify") {
    //     Some(s) => Some(
    //         s.parse()
    //             .map_err(|_| "Invalid blocknotify commmand".to_owned())?,
    //     ),
    //     None => None,
    // };

    // let services = Services::default().with_network(true);
    // let services = match &consensus.fork {
    //     &ConsensusFork::BitcoinCash(_) => services.with_bitcoin_cash(true),
    //     &ConsensusFork::BitcoinCore => services.with_witness(true),
    // };

    // let verification_level = match matches.value_of("verification-level") {
    //     Some(s) if s == "full" => VerificationLevel::Full,
    //     Some(s) if s == "header" => VerificationLevel::Header,
    //     Some(s) if s == "none" => VerificationLevel::NoVerification,
    //     Some(s) => return Err(format!("Invalid verification level: {}", s)),
    //     None => VerificationLevel::Full,
    // };

    // let verification_edge = match matches.value_of("verification-edge") {
    //     Some(s) if verification_level != VerificationLevel::Full => {
    //         let edge: H256 = s
    //             .parse()
    //             .map_err(|_| "Invalid verification edge".to_owned())?;
    //         edge.reversed()
    //     }
    //     _ => network.default_verification_edge(),
    // };

    // let config = Config {
    //     quiet: quiet,
    //     network: network,
    //     consensus: consensus,
    //     services: services,
    //     port: port,
    //     connect: connect,
    //     host: host,
    //     seednodes: seednodes,
    //     inbound_connections: in_connections,
    //     outbound_connections: out_connections,
    //     p2p_threads: p2p_threads,
    //     db_cache: db_cache,
    //     data_dir: data_dir,
    //     user_agent: user_agent,
    //     internet_protocol: only_net,
    //     rpc_config: rpc_config,
    //     block_notify_command: block_notify_command,
    //     verification_params: VerificationParameters {
    //         verification_level: verification_level,
    //         verification_edge: verification_edge,
    //     },
    //     db: db,
    // };

    // Ok(config)
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

// fn parse_consensus_fork(
//     network: Network,
//     db: &storage::SharedStore,
//     matches: &clap::ArgMatches,
// ) -> Result<ConsensusFork, String> {
//     let old_consensus_fork = db.consensus_fork()?;
//     let new_consensus_fork = match (matches.is_present("btc"), matches.is_present("bch")) {
//         (false, false) => match &old_consensus_fork {
//             &Some(ref old_consensus_fork) => old_consensus_fork,
//             &None => return Err("You must select fork on first run: --btc, --bch".into()),
//         },
//         (true, false) => "btc",
//         (false, true) => "bch",
//         _ => return Err("You can only pass single fork argument: --btc, --bch".into()),
//     };

//     match &old_consensus_fork {
//         &None => db.set_consensus_fork(new_consensus_fork)?,
//         &Some(ref old_consensus_fork) if old_consensus_fork == new_consensus_fork => (),
//         &Some(ref old_consensus_fork) => {
//             return Err(format!(
//                 "Cannot select '{}' fork with non-empty database of '{}' fork",
//                 new_consensus_fork, old_consensus_fork
//             ))
//         }
//     }

//     return match new_consensus_fork {
//         "btc" => Ok(ConsensusFork::BitcoinCore),
//         "bch" => Ok(ConsensusFork::BitcoinCash(BitcoinCashConsensusParams::new(
//             network,
//         ))),
//         _ => Err(String::from("Fork mandatory")),
//     };
// }

// fn parse_rpc_config(network: Network, matches: &clap::ArgMatches) -> Result<RpcHttpConfig, String> {
//     let mut config = RpcHttpConfig::with_port(network.rpc_port());
//     config.enabled = !matches.is_present("no-jsonrpc");
//     if !config.enabled {
//         return Ok(config);
//     }

//     if let Some(apis) = matches.value_of("jsonrpc-apis") {
//         config.apis = ApiSet::List(
//             vec![apis.parse().map_err(|_| "Invalid APIs".to_owned())?]
//                 .into_iter()
//                 .collect(),
//         );
//     }
//     if let Some(port) = matches.value_of("jsonrpc-port") {
//         config.port = port
//             .parse()
//             .map_err(|_| "Invalid JSON RPC port".to_owned())?;
//     }
//     if let Some(interface) = matches.value_of("jsonrpc-interface") {
//         config.interface = interface.to_owned();
//     }
//     if let Some(cors) = matches.value_of("jsonrpc-cors") {
//         config.cors = Some(vec![cors
//             .parse()
//             .map_err(|_| "Invalid JSON RPC CORS".to_owned())?]);
//     }
//     if let Some(hosts) = matches.value_of("jsonrpc-hosts") {
//         config.hosts = Some(vec![hosts
//             .parse()
//             .map_err(|_| "Invalid JSON RPC hosts".to_owned())?]);
//     }

//     Ok(config)
// }
