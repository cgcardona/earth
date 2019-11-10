use crate::{MIN, VERSION};
use configuration::Configuration;
use database::DataBase;
use eventloop::EventLoop;
use p2p::Config;
use p2p::NetworkConfig;
use p2p::{IP, P2P};
use services::Services;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use synchronization::Synchronization;

/// start EARTH client with command line arguments
pub fn start(config: Configuration) -> Result<(), String> {
    let eventloop: EventLoop = EventLoop::new();

    DataBase::init(&config);

    let node_table_path: PathBuf = match config.data_dir {
        Some(ref data_dir) => {
            P2P::create_p2p_dir(&data_dir, "p2p");
            DataBase::create_data_dir(&data_dir, "db")
        }
        None => {
            P2P::create_p2p_dir("data-dir", "p2p");
            DataBase::create_data_dir("data-dir", "db")
        }
    };

    let outbound_connections: u32 = config.outbound_connections;
    let inbound_connections: u32 = config.inbound_connections;
    let threads: usize = config.threads;
    let node_table_path: PathBuf = node_table_path;
    let seeds: Vec<String> = config.seeders;
    let peers: Vec<SocketAddr> = config.connect.map_or_else(|| vec![], |x| vec![x]);
    let ip: IP = config.ip;
    let protocol_version: u32 = VERSION;
    let protocol_minimum: u32 = MIN;
    let ua: String = config.ua;
    let start_height: i32 = 0;
    let relay: bool = true;
    let magic: u32 = config.consensus.magic();
    let local_address: SocketAddr = SocketAddr::new(config.host, config.port);
    let services: Services = config.services;

    let connection: NetworkConfig = NetworkConfig {
        protocol_version: protocol_version,
        protocol_minimum: protocol_minimum,
        ua: ua,
        start_height: start_height,
        relay: relay,
        magic: magic,
        local_address: local_address,
        services: services.clone(),
    };

    let c: Config = Config::new(
        outbound_connections,
        inbound_connections,
        threads,
        node_table_path,
        seeds,
        peers,
        ip,
        services,
        connection,
    );

    let sync_peers: Arc<u8> = Synchronization::create_peers().unwrap();

    let local_sync_node: () = Synchronization::create_sync_node(
        config.consensus,
        config.db.clone(),
        sync_peers.clone(),
        config.verification_params,
    )
    .unwrap();

    // let sync_connection_factory =
    //     create_sync_connect.handle()ion_factory(sync_peers.clone(), local_sync_node.clone());
    // struct Foo {}
    // impl LocalSyncNode for Foo {
    //     fn create_sync_session(&self) {}
    // }

    // let localSyncNode: Foo = Foo {};

    // let sync_connection_factory: LocalSyncNodeRef = Box::new(localSyncNode);

    // let p2p: P2P = P2P::new(config, sync_connection_factory, handle).unwrap();
    // for seed in p2p.config.seeds {
    //     dns_lookup(seed);
    // }

    Ok(())
}
