use abstract_ns::{Address, HostResolve, IpList, Resolve};
use ns_dns_tokio::DnsResolver;
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
