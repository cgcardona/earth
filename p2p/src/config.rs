use network::Magic;
use services::Services;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    outbound_connections: u32,
    inbound_connections: u32,
    pub threads: usize,
    node_table_path: PathBuf,
    seeds: Vec<String>,
    peers: Vec<SocketAddr>,
    ip: IP,
    services: Services,
    connection: NetConfig,
}

impl Config {
    pub fn new(
        outbound_connections: u32,
        inbound_connections: u32,
        threads: usize,
        node_table_path: PathBuf,
        seeds: Vec<String>,
        peers: Vec<SocketAddr>,
        ip: IP,
        services: Services,
        connection: NetConfig,
    ) -> Self {
        Config {
            outbound_connections: outbound_connections,
            inbound_connections: inbound_connections,
            threads: threads,
            node_table_path: node_table_path,
            seeds: seeds,
            peers: peers,
            ip: ip,
            services: services,
            connection: connection,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IP {
    IPV4,
    IPV6,
}

#[derive(Debug, Clone)]
pub struct NetConfig {
    pub protocol_version: u32,
    pub protocol_minimum: u32,
    pub magic: Magic,
    pub local_address: SocketAddr,
    pub services: Services,
    pub ua: String,
    pub start_height: i32,
    pub relay: bool,
}
