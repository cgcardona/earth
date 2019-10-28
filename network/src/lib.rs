#[macro_use]
extern crate lazy_static;
extern crate chain;
extern crate primitives;

mod consensus;
mod deployments;
mod network;

pub use chain::IndexedBlock;
pub use primitives::{compact, hash};

pub use consensus::{
    BitcoinCashConsensusParams, ConsensusFork, ConsensusParams, TransactionOrdering,
};
pub use deployments::Deployment;
pub use network::{Magic, Network};
