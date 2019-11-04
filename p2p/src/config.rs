use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub outbound_connections: u32,
    pub inbound_connections: u32,
    pub threads: u32,
    pub node_table_path: PathBuf,
    pub seeds: Vec<String>,
    pub peers: Vec<SocketAddr>,
    pub internet_protocol: IP,
    // pub preferable_services: Services,
    pub connection: NetConfig,
}

#[derive(Debug)]
pub enum IP {
    IPV4,
    IPV6,
}

// #[derive(Debug)]
// pub struct Services(u64);

#[derive(Debug)]
pub struct NetConfig {
    pub protocol_version: u32,
    pub protocol_minimum: u32,
    // pub magic: Magic,
    // pub local_address: SocketAddr,
    // pub services: Services,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}
