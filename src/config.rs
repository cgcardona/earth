use clap;
// use message::Services;
use network::{BitcoinCashConsensusParams, ConsensusFork, ConsensusParams, Network};
use p2p::InternetProtocol;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use primitives::hash::H256;
use crate::rpc::HttpConfiguration as RpcHttpConfig;
use crate::rpc_apis::ApiSet;
use crate::seednodes::{
    bitcoin_cash_seednodes, bitcoin_cash_testnet_seednodes, mainnet_seednodes, testnet_seednodes,
};
use std::net;
// use storage;
// use sync::VerificationParameters;
// use super::util::open_db;
// use verification::VerificationLevel;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
pub struct Config {
    // pub services: Services,
    // pub internet_protocol: InternetProtocol,
    // pub verification_params: VerificationParameters,
    // pub db: storage::SharedStore,
    pub rpc_config: RpcHttpConfig,
    pub seednodes: Vec<String>,
    pub connect: Option<net::SocketAddr>,
    pub consensus: ConsensusParams,
    pub block_notify_command: Option<String>,
    pub host: net::IpAddr,
    pub port: u16,
    pub p2p_threads: usize,
    pub inbound_connections: u32,
    pub outbound_connections: u32,
    pub db_cache: usize,
    pub network: Network,
    pub services: String,
    pub quiet: bool,
    pub data_dir: Option<String>,
    pub user_agent: String,
    pub internet_protocol: String,
    pub verification_params: String,
    // pub db: String,
}

pub const DEFAULT_DB_CACHE: usize = 512;

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
    let db_cache: usize = match matches.value_of("db-cache") {
        Some(s) => s
            .parse()
            .map_err(|_| "Invalid cache size - should be number in MB".to_owned())?,
        None => DEFAULT_DB_CACHE,
    };

    let data_dir: Option<String> = match matches.value_of("data-dir") {
        Some(s) => Some(s.parse().map_err(|_| "Invalid data-dir".to_owned())?),
        None => None,
    };

    // let db = open_db(&data_dir, db_cache);

    let quiet: bool = matches.is_present("quiet");

    let network: Network = match (matches.is_present("testnet"), matches.is_present("regtest")) {
        (true, false) => Network::Testnet,
        (false, true) => Network::Regtest,
        (false, false) => Network::Mainnet,
        (true, true) => return Err("Only one testnet option can be used".into()),
    };

    let fork: &str = "bch";
    let consensus_fork = parse_consensus_fork(network, fork)?;
    let consensus: ConsensusParams = ConsensusParams::new(network, consensus_fork);

    let (in_connections, out_connections): (u32, u32) = match network {
        Network::Testnet | Network::Mainnet | Network::Other(_) => (10, 10),
        Network::Regtest | Network::Unitest => (1, 0),
    };

    let p2p_threads: usize = match network {
        Network::Testnet | Network::Mainnet | Network::Other(_) => 4,
        Network::Regtest | Network::Unitest => 1,
    };

    let user_agent: String = String::from("/EARTH:0.0.1/");

    let port: u16 = match matches.value_of("port") {
        Some(port) => port.parse().map_err(|_| "Invalid port".to_owned())?,
        None => network.port(),
    };

    let connect: Option<net::SocketAddr> = match matches.value_of("connect") {
        Some(s) => Some(match s.parse::<net::SocketAddr>() {
            Err(_) => s
                .parse::<net::IpAddr>()
                .map(|ip| net::SocketAddr::new(ip, network.port()))
                .map_err(|_| "Invalid connect".to_owned()),
            Ok(a) => Ok(a),
        }?),
        None => None,
    };

    let seednodes: Vec<String> = match matches.value_of("seednode") {
        Some(s) => vec![s.parse().map_err(|_| "Invalid seednode".to_owned())?],
        None => match (network, &consensus.fork) {
            (Network::Mainnet, &ConsensusFork::BitcoinCash(_)) => bitcoin_cash_seednodes()
                .into_iter()
                .map(Into::into)
                .collect(),
            (Network::Testnet, &ConsensusFork::BitcoinCash(_)) => bitcoin_cash_testnet_seednodes()
                .into_iter()
                .map(Into::into)
                .collect(),
            (Network::Mainnet, _) => mainnet_seednodes().into_iter().map(Into::into).collect(),
            (Network::Testnet, _) => testnet_seednodes().into_iter().map(Into::into).collect(),
            (Network::Other(_), _) | (Network::Regtest, _) | (Network::Unitest, _) => Vec::new(),
        },
    };
    println!("SEEDNODES: {:#?}", matches);

    let only_net: p2p::InternetProtocol = match matches.value_of("only-net") {
        Some(s) => s.parse()?,
        None => InternetProtocol::default(),
    };

    let host: std::net::IpAddr = match matches.value_of("host") {
        Some(s) => s
            .parse::<net::IpAddr>()
            .map_err(|_| "Invalid host".to_owned())?,
        None => match only_net {
            InternetProtocol::IpV6 => "::".parse().unwrap(),
            _ => "0.0.0.0".parse().unwrap(),
        },
    };

    let rpc_config = parse_rpc_config(network, matches)?;

    let block_notify_command: Option<String> = match matches.value_of("blocknotify") {
        Some(s) => Some(
            s.parse()
                .map_err(|_| "Invalid blocknotify commmand".to_owned())?,
        ),
        None => None,
    };

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
    let services: String = String::from("services");
    let internet_protocol: String = String::from("internet_protocol");
    let verification_params: String = String::from("verification_params");

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
        inbound_connections: in_connections,
        outbound_connections: out_connections,
        p2p_threads: p2p_threads,
        user_agent: user_agent,
        internet_protocol: internet_protocol,
        rpc_config: rpc_config,
        block_notify_command: block_notify_command,
        verification_params: verification_params,
        // db: db,
    })
}

fn parse_consensus_fork(network: Network, fork: &str) -> Result<ConsensusFork, String> {
    return match fork {
        "btc" => Ok(ConsensusFork::BitcoinCore),
        "bch" => Ok(ConsensusFork::BitcoinCash(BitcoinCashConsensusParams::new(
            network,
        ))),
        _ => Err(String::from("Fork mandatory")),
    };
}

fn parse_rpc_config(network: Network, matches: &clap::ArgMatches) -> Result<RpcHttpConfig, String> {
    let mut config = RpcHttpConfig::with_port(network.rpc_port());
    config.enabled = !matches.is_present("no-jsonrpc");
    if !config.enabled {
        return Ok(config);
    }

    if let Some(apis) = matches.value_of("jsonrpc-apis") {
        config.apis = ApiSet::List(
            vec![apis.parse().map_err(|_| "Invalid APIs".to_owned())?]
                .into_iter()
                .collect(),
        );
    }
    if let Some(port) = matches.value_of("jsonrpc-port") {
        config.port = port
            .parse()
            .map_err(|_| "Invalid JSON RPC port".to_owned())?;
    }
    if let Some(interface) = matches.value_of("jsonrpc-interface") {
        config.interface = interface.to_owned();
    }
    if let Some(cors) = matches.value_of("jsonrpc-cors") {
        config.cors = Some(vec![cors
            .parse()
            .map_err(|_| "Invalid JSON RPC CORS".to_owned())?]);
    }
    if let Some(hosts) = matches.value_of("jsonrpc-hosts") {
        config.hosts = Some(vec![hosts
            .parse()
            .map_err(|_| "Invalid JSON RPC hosts".to_owned())?]);
    }

    Ok(config)
}
