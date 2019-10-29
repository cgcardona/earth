use crate::common::PrefilledTransaction;
use chain::{BlockHeader, ShortTransactionID};

#[derive(Debug, PartialEq)]
pub struct BlockHeaderAndIDs {
	pub header: BlockHeader,
	pub nonce: u64,
	pub short_ids: Vec<ShortTransactionID>,
	pub prefilled_transactions: Vec<PrefilledTransaction>,
}
