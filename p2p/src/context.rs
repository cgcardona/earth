use crate::Config;
use futures_cpupool::CpuPool;
use parking_lot::RwLock;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;

/// Network context.
pub struct Context {
    pool: CpuPool,
    // node_table: RwLock<NodeTable>,
    config: Config,
}

pub struct NodeTable {}

impl Context {
    // pub fn new(pool: CpuPool, handle: Handle, node_table: RwLock<NodeTable>, config: Config) -> Self {
    pub fn new(pool: CpuPool, handle: Handle, config: Config) -> Self {
        Context {
            pool: pool,
            // node_table: node_table,
            config: config,
        }
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
