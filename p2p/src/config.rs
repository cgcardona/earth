use services::Services;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub outbound_connections: u32,
    pub inbound_connections: u32,
    pub threads: usize,
    pub node_table_path: PathBuf,
    pub seeds: Vec<String>,
    pub peers: Vec<SocketAddr>,
    pub ip: IP,
    pub services: Services,
    pub connection: NetConfig,
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
    // pub magic: Magic,
    pub local_address: SocketAddr,
    pub services: Services,
    pub ua: String,
    pub start_height: i32,
    pub relay: bool,
}
