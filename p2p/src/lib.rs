extern crate abstract_ns;
extern crate domain;
extern crate futures;
extern crate ns_dns_tokio;
extern crate tokio_core;

mod config;
mod consensus;
mod dns;
pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};
pub use dns::dns_lookup;

pub struct P2P {
    pub config: Config,
}

impl P2P {
    pub fn new(p2p_config: Config) -> Self {
        P2P { config: p2p_config }
    }
}
