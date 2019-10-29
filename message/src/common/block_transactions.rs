use crate::hash::H256;
use chain::Transaction;
use ser::{Deserializable, Error as ReaderError, Reader, Serializable, Stream};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, PartialEq)]
pub struct BlockTransactions {
	pub blockhash: H256,
	pub transactions: Vec<Transaction>,
}

// TODO this is a placeholder Serializable impl
impl Serializable for BlockTransactions {
	fn serialize(&self, stream: &mut Stream) {}
}

// // TODO this is a placeholder Deserializable impl
impl Deserializable for BlockTransactions {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
	where
		T: io::Read,
	{
		let data = r#try!(reader.read::<BlockTransactions>());

		Ok(data)
	}
}
