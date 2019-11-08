use futures_cpupool::{Builder, CpuPool};
use std::sync::Arc;
use tokio_core::reactor::Handle;
// use parking_lot::RwLock;
// use std::net::SocketAddr;

mod config;
mod consensus;
mod context;
mod dns;

pub use config::{Config, NetConfig, IP};
pub use consensus::{ConsensusParams, Deployment};
pub use context::{Context, NodeTable};
pub use dns::dns_lookup;

#[derive(Debug)]
pub struct P2P {
    pub config: Config,
    // pub context: Arc<Context>,
}

impl P2P {
    // pub fn new(config: Config, local_sync_node: LocalSyncNodeRef, handle: Handle) -> Result<Self, Box<dyn error::Error>> {
    pub fn new(config: Config, handle: Handle) -> Self {
        let pool: CpuPool = Builder::new()
            .name_prefix("I/O thread")
            .pool_size(config.threads)
            .create();

        // let context = try!(Context::new(local_sync_node, pool.clone(), handle.remote().clone(), config.clone()));
        let context = Context::new(pool, handle, config.clone());
        P2P {
            config: config,
            // context: context,
        }
    }
}
