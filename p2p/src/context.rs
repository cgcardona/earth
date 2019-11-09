use crate::local_sync_node::LocalSyncNodeRef;
use crate::Config;
use futures_cpupool::CpuPool;
use std::error;
use std::net::SocketAddr;
use tokio_core::reactor::Remote;
// use parking_lot::RwLock;

/// Network context.
pub struct Context {
    pool: CpuPool,
    remote: Remote,
    // node_table: RwLock<NodeTable>,
    config: Config,
}

pub struct NodeTable {}

impl Context {
    pub fn new(
        local_sync_node: LocalSyncNodeRef,
        pool: CpuPool,
        remote: Remote,
        config: Config,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(Context {
            pool: pool,
            // node_table: node_table,
            remote: remote,
            config: config,
        })
    }

    /// Add a node to the noe_table
    pub fn add_node(&self, address: SocketAddr) -> Result<(), ()> {
        // self.config
        //     .node_table_path
        //     .write()
        //     .add(address, self.config.connection.services);
        Ok(())
    }

    /// Remove a node from the node_table
    pub fn remove_node(&self, address: SocketAddr) -> Result<(), ()> {
        // self.config.node_table_path.write().remove(&address)
        Ok(())
    }
}
