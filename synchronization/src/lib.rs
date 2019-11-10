use p2p::ConsensusParams;
use std::sync::Arc;
pub struct Synchronization {}

impl Synchronization {
    // pub fn create_sync_peers() -> PeersRef {
    pub fn create_peers() -> Result<Arc<u8>, ()> {
        // use synchronization_peers::PeersImpl;

        Ok(Arc::new(0))
    }

    pub fn create_sync_node(
        consensus: ConsensusParams,
        // db: storage::SharedStore,
        // peers: PeersRef,
        // verification_params: VerificationParameters,
        // ) -> LocalNodeRef {
    ) -> Result<(), ()> {
        Ok(())
        // use miner::MemoryPool;
        // use synchronization_chain::Chain as SyncChain;
        // use synchronization_executor::LocalSynchronizationTaskExecutor as SyncExecutor;
        // use local_node::LocalNode as SyncNode;
        // use synchronization_server::ServerImpl;
        // use synchronization_client::SynchronizationClient;
        // use synchronization_client_core::{SynchronizationClientCore, CoreVerificationSink, Config as SynchronizationConfig};
        // use synchronization_verifier::AsyncVerifier;
        // use utils::SynchronizationState;
        // use types::SynchronizationStateRef;

        // let network = consensus.network;
        // let sync_client_config = SynchronizationConfig {
        // 	// during regtests, peer is providing us with bad blocks => we shouldn't close connection because of this
        // 	close_connection_on_bad_block: network != Network::Regtest,
        // };
        // let mut memory_pool = MemoryPool::new();
        // if network == Network::Regtest {
        // 	// during regtests, peer is providing us with zero fee transactions => we shouldn't ignore these
        // 	memory_pool.accept_zero_fee_transactions();
        // }

        // let memory_pool = Arc::new(RwLock::new(memory_pool));
        // let sync_state = SynchronizationStateRef::new(SynchronizationState::with_storage(db.clone()));
        // let sync_chain = SyncChain::new(db.clone(), consensus.clone(), memory_pool.clone());
        // if sync_chain.is_segwit_possible() {
        // 	peers.require_peer_services(Services::default().with_witness(true));
        // }

        // let chain_verifier = Arc::new(ChainVerifier::new(db.clone(), consensus.clone()));
        // let sync_executor = SyncExecutor::new(peers.clone());
        // let sync_server = Arc::new(ServerImpl::new(peers.clone(), db.clone(), memory_pool.clone(), sync_executor.clone()));
        // let sync_client_core = SynchronizationClientCore::new(sync_client_config, sync_state.clone(), peers.clone(), sync_executor.clone(), sync_chain, chain_verifier.clone());
        // let verifier_sink = Arc::new(CoreVerificationSink::new(sync_client_core.clone()));
        // let verifier = AsyncVerifier::new(chain_verifier, db.clone(), memory_pool.clone(), verifier_sink, verification_params);
        // let sync_client = SynchronizationClient::new(sync_state.clone(), sync_client_core, verifier);
        // Arc::new(SyncNode::new(consensus, db, memory_pool, peers, sync_state, sync_client, sync_server))
    }
}
