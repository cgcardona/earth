use ns_dns_tokio::DnsResolver;
use tokio_core::reactor::{Core, Handle};

mod config;
mod consensus;
pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};

pub struct P2P {
    handle: Handle,
}

impl P2P {
    pub fn dns_lookup(p2p_config: Config) {
        // let resolver: Result<DnsResolver, _> = DnsResolver::system_config(&p2p_config);
        // for seed in seeders {
        //     connect_to_seednode(seed);
        // }
    }

    // pub fn connect_to_seednode(seed: &String) {
    //     println!("{:#?}", seed);
    // }
}

pub fn event_loop() -> Core {
    Core::new().unwrap()
}
