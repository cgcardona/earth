use crate::Util;
use configuration::Configuration;
use p2p::Config;
use p2p::{dns_lookup, P2P};
use p2p::{LocalSyncNode, LocalSyncNodeRef};
use std::{fs, path::PathBuf};
use tokio_core::reactor::{Core, Handle};

/// start EARTH client with command line arguments
pub fn start(config: Configuration) -> Result<(), String> {
    let mut event_loop: Core = Core::new().unwrap();

    Util::initialize_database(&config);

    let node_table_path: PathBuf = Util::node_table_path(&config);

    let c: Config = Config {
        outbound_connections: config.outbound_connections,
        inbound_connections: config.inbound_connections,
        threads: config.threads,
        node_table_path: node_table_path,
        seeds: config.seeders.clone(),
        peers: config.connect.map_or_else(|| vec![], |x| vec![x]),
        ip: config.ip,
        connection: p2p::NetConfig {
            protocol_version: 70_014,
            protocol_minimum: 70_001,
            // magic: cfg.consensus.magic(),
            // local_address: SocketAddr::new(c.host, c.port),
            // services: c.services,
            user_agent: config.user_agent,
            start_height: 0,
            relay: true,
        },
    };

    // let sync_connection_factory =
    //     create_sync_connect.handle()ion_factory(sync_peers.clone(), local_sync_node.clone());
    struct Foo {}
    impl LocalSyncNode for Foo {
        fn create_sync_session(&self) {}
    }

    let localSyncNode: Foo = Foo {};

    let sync_connection_factory: LocalSyncNodeRef = Box::new(localSyncNode);

    // let p2p: P2P = P2P::new(config, sync_connection_factory, handle).unwrap();
    // for seed in p2p.config.seeds {
    //     dns_lookup(seed);
    // }

    Ok(())
}
