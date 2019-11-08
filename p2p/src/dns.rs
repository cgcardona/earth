// TODO - Use return Address from Resolve instead of returning IPList from HostResolve.
// Currently host_resolve is only parsing hostnames while host() will parse hostname and port.
// use abstract_ns::{Address, HostResolve, IpList, Resolve};
use abstract_ns::{HostResolve, IpList};
use ns_dns_tokio::DnsResolver;
use std::net::SocketAddr;
use tokio_core::net::TcpStream;
use tokio_core::reactor::{Core, Handle};

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
                // TODO - Support mainnet with port 8333
                connect_to_ip(SocketAddr::new(sock, 18333), handle);
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

pub fn connect_to_ip(sock: SocketAddr, handle: Handle) {
    TcpStream::connect(&sock, &handle);
}
