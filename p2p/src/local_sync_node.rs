pub type LocalSyncNodeRef = Box<dyn LocalSyncNode>;
// pub trait LocalSyncNode : Send + Sync {
// 	fn create_sync_session(&self, height: i32, services: Services, outbound: OutboundSyncConnectionRef) -> InboundSyncConnectionRef;
// }
pub trait LocalSyncNode {
    fn create_sync_session(&self);
}
