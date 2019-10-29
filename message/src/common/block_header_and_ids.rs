use crate::common::PrefilledTransaction;
use chain::{BlockHeader, ShortTransactionID};
use ser::{Deserializable, Error as ReaderError, Reader, Serializable, Stream};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, PartialEq)]
pub struct BlockHeaderAndIDs {
	pub header: BlockHeader,
	pub nonce: u64,
	pub short_ids: Vec<ShortTransactionID>,
	pub prefilled_transactions: Vec<PrefilledTransaction>,
}

// TODO this is a placeholder Serializable impl
impl Serializable for BlockHeaderAndIDs {
	fn serialize(&self, stream: &mut Stream) {}
}

// // TODO this is a placeholder Deserializable impl
impl Deserializable for BlockHeaderAndIDs {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
	where
		T: io::Read,
	{
		let data = r#try!(reader.read::<BlockHeaderAndIDs>());

		Ok(data)
	}
}
