extern crate abstract_ns;
extern crate domain;
extern crate futures;
extern crate ns_dns_tokio;
extern crate tokio_core;

use futures_cpupool::{Builder, CpuPool};
use std::net::SocketAddr;

mod config;
mod consensus;
mod context;
mod dns;

pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};
pub use context::Context;
pub use dns::dns_lookup;

pub struct P2P {
    pub config: Config,
}

impl P2P {
    pub fn new(p2p_config: Config) -> Self {
        let pool: CpuPool = Builder::new()
            .name_prefix("I/O thread")
            .pool_size(p2p_config.threads)
            .create();

        let context = Context::new(pool);
        P2P { config: p2p_config }
    }

    /// Add a node to the noe_table
    pub fn add_node(&self, address: SocketAddr) -> Result<(), ()> {
        // self.config
        //     .node_table_path
        //     .write()
        //     .add(address, self.config.connection.services)
        Ok(())
    }

    /// Remove a node from the node_table
    pub fn remove_node(&self, address: SocketAddr) -> Result<(), ()> {
        // self.config.node_table_path.write().remove(&address)
        Ok(())
    }
}
