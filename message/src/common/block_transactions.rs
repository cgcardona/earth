use crate::hash::H256;
use chain::Transaction;

#[derive(Debug, PartialEq)]
pub struct BlockTransactions {
	pub blockhash: H256,
	pub transactions: Vec<Transaction>,
}
