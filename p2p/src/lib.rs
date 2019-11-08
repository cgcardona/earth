extern crate abstract_ns;
extern crate argparse;
extern crate domain;
extern crate futures;
extern crate ns_dns_tokio;
extern crate tokio_core;

use tokio_core::reactor::{Core, Handle};

use abstract_ns::HostResolve;
use argparse::{ArgumentParser, Store};
use ns_dns_tokio::DnsResolver;

mod config;
mod consensus;
pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};

pub struct P2P {
    handle: Handle,
}

impl P2P {
    pub fn dns_lookup(p2p_config: Config) {
        let mut core = Core::new().unwrap();
        let resolver =
            DnsResolver::system_config(&core.handle()).expect("initializing DNS resolver");
        let res =
            core.run(resolver.resolve_host(&"testnet-seed-abc.bitcoinforks.org".parse().unwrap()));
        match res {
            Ok(address) => match address.pick_one() {
                Some(socket) => {
                    println!("{:#?}", socket);
                }
                None => {
                    println!("None");
                }
            },
            Err(_err) => {
                println!("Error: {:#?}", _err);
            }
        }
    }
}

pub fn event_loop() -> Core {
    Core::new().unwrap()
}
