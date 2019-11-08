extern crate abstract_ns;
extern crate domain;
extern crate futures;
extern crate ns_dns_tokio;
extern crate tokio_core;

use tokio_core::reactor::{Core, Handle};

use abstract_ns::{HostResolve, IpList};
use ns_dns_tokio::DnsResolver;

mod config;
mod consensus;
pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};

pub struct P2P {
    handle: Handle,
}

impl P2P {
    pub fn dns_lookup(seed: String) {
        let mut core: Core = Core::new().unwrap();
        let handle: Handle = core.handle();
        let resolver: DnsResolver =
            DnsResolver::system_config(&handle).expect("initializing DNS resolver");
        let res: Result<IpList, abstract_ns::Error> =
            core.run(resolver.resolve_host(&seed.parse().unwrap()));
        match res {
            Ok(addr) => match addr.pick_one() {
                Some(sock) => {
                    println!("{:#?}", sock);
                }
                None => {
                    println!("None");
                }
            },
            Err(err) => {
                println!("Error: {:#?}", err);
            }
        }
    }
}
