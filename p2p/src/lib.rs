use futures_cpupool::{Builder, CpuPool};
use std::error;
use std::sync::Arc;
use std::{fs, path::PathBuf};
use tokio_core::reactor::Handle;
// use parking_lot::RwLock;
// use std::net::SocketAddr;

mod config;
mod consensus;
mod context;
mod dns;
mod local_sync_node;

pub use config::{Config, NetworkConfig, IP};
pub use consensus::{ConsensusParams, Deployment};
pub use context::{Context, NodeTable};
pub use dns::dns_lookup;
pub use local_sync_node::{LocalSyncNode, LocalSyncNodeRef};

#[derive(Debug)]
pub struct P2P {
    pub config: Config,
    pub handle: Handle,
    pub pool: CpuPool,
    pub context: Arc<Context>,
}

impl P2P {
    pub fn new(
        config: Config,
        local_sync_node: LocalSyncNodeRef,
        handle: Handle,
    ) -> Result<Self, Box<dyn error::Error>> {
        let pool: CpuPool = Builder::new()
            .name_prefix("I/O thread")
            .pool_size(config.threads)
            .create();

        let context = Context::new(
            local_sync_node,
            pool.clone(),
            handle.remote().clone(),
            config.clone(),
        )
        .unwrap();

        Ok(P2P {
            config: config,
            handle: handle,
            pool: pool,
            context: Arc::new(context),
        })
    }

    /// create p2p directory if it doesn't exist
    pub fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
        let p: PathBuf = [p2p, sub].iter().collect();

        fs::create_dir_all(&p).expect("Failed to get p2p dir");

        p
    }
}
