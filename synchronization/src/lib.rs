use std::sync::Arc;
pub struct Synchronization {}

impl Synchronization {
    // pub fn create_sync_peers() -> PeersRef {
    pub fn create_peers() -> Result<Arc<u8>, ()> {
        // Ok(())
        // use synchronization_peers::PeersImpl;

        Ok(Arc::new(0))
    }
}
