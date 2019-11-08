use crate::Configuration;
use blockchain::Block;
use database::Storage;
use mock_data::block_mock_data;
use p2p::Config;
use p2p::{NetConfig, P2P};
use std::{fs, path::PathBuf};
use tokio_core::reactor::{Core, Handle};

// use crate::seeders::{mainnet_seeders, testnet_seeders};
// use blockchain::Block;
// use database::*;
// use mock_data::block_mock_data;

/// imports blockchain data
pub fn import(c: &Configuration, m: &clap::ArgMatches) {
    let g: &str = c.network.genesis();
    let i: &str = m.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", m, g, i);
}

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, m: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", c, m);
}

/// start EARTH client with command line arguments
pub fn start(c: Configuration) {
    start_db(&c);
    start_p2p(c);
}

/// Start database setup
fn start_db(c: &Configuration) {
    // create db directory
    match c.data_dir {
        Some(ref data_dir) => create_data_dir(&data_dir, "db"),
        None => create_data_dir("data-dir", "db"),
    };

    let data_dir: String = match c.data_dir {
        Some(ref data_dir) => String::from(data_dir),
        None => String::from("data-dir"),
    };

    // let s: Storage = Storage::new(data_dir);

    // let b: Block = Block::new(None, None);

    // for x in 0..32 {
    //     let b: Block = mock_data::block_mock_data(x);
    //     // println!("Block: {:#?}", b);
    //     let key: &str = &format!("{}{}", "foo", x);
    //     // println!("Key: {:#?}", key);
    //     let serialized0 = serde_json::to_string(&b).unwrap();
    //     assert!(s.write(key, serialized0).is_ok());
    //     assert!(s.read(key).is_ok());
    //     assert!(s.delete(key).is_ok());
    // }
}

/// Start p2p connections
fn start_p2p(c: Configuration) {
    let mut el: Core = p2p::event_loop();

    // create p2p directory
    let node_table_path: PathBuf = match c.data_dir {
        Some(ref data_dir) => create_p2p_dir(&data_dir, "p2p"),
        None => create_p2p_dir("data-dir", "p2p"),
    };

    // let p2p_cfg = p2p::Config {
    //     connection: p2p::NetConfig {
    //         magic: cfg.consensus.magic(),
    //         local_address: SocketAddr::new(cfg.host, cfg.port),
    //         services: cfg.services,
    //     },
    // };

    let p2p_config: Config = Config {
        outbound_connections: c.outbound_connections,
        inbound_connections: c.inbound_connections,
        threads: c.threads,
        node_table_path: node_table_path,
        seeds: c.seeders.clone(),
        peers: c.connect.map_or_else(|| vec![], |x| vec![x]),
        internet_protocol: c.internet_protocol,
        connection: p2p::NetConfig {
            protocol_version: 70_014,
            protocol_minimum: 70_001,
            // magic: cfg.consensus.magic(),
            // local_address: SocketAddr::new(c.host, c.port),
            // services: c.services,
            user_agent: c.user_agent,
            start_height: 0,
            relay: true,
        },
    };

    P2P::dns_lookup(p2p_config);
}

/// create data_dir if it doesn't exist
fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [data_dir, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get app dir");

    p
}

/// create p2p directory if it doesn't exist
fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [p2p, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get p2p dir");

    p
}
