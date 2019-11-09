use configuration::Configuration;
use p2p::Config;
use p2p::{dns_lookup, P2P};
use p2p::{LocalSyncNode, LocalSyncNodeRef};
use std::{fs, path::PathBuf};
use tokio_core::reactor::{Core, Handle};

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

    // let data_dir: String = match c.data_dir {
    //     Some(ref data_dir) => String::from(data_dir),
    //     None => String::from("data-dir"),
    // };
}

/// Start p2p connections
fn start_p2p(c: Configuration) {
    let mut core: Core = Core::new().unwrap();

    let handle: Handle = core.handle();

    // create p2p directory
    let mut node_table_path: PathBuf = match c.data_dir {
        Some(ref data_dir) => create_p2p_dir(&data_dir, "p2p"),
        None => create_p2p_dir("data-dir", "p2p"),
    };

    node_table_path.push("nodes.csv");

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

    // let sync_connection_factory =
    //     create_sync_connect.handle()ion_factory(sync_peers.clone(), local_sync_node.clone());
    struct Foo {}
    impl LocalSyncNode for Foo {
        fn create_sync_session(&self) {}
    }

    let localSyncNode: Foo = Foo {};

    let sync_connection_factory: LocalSyncNodeRef = Box::new(localSyncNode);

    let p2p: P2P = P2P::new(p2p_config, sync_connection_factory, handle).unwrap();
    for seed in p2p.config.seeds {
        dns_lookup(seed);
    }
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
