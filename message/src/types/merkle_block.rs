use crate::bytes::Bytes;
use crate::hash::H256;
use crate::{MessageResult, Payload};
use chain::BlockHeader;
use ser::{Reader, Stream};
use std::io;

#[derive(Debug, PartialEq)]
pub struct MerkleBlock {
	pub block_header: BlockHeader,
	pub total_transactions: u32,
	pub hashes: Vec<H256>,
	pub flags: Bytes,
}

impl Payload for MerkleBlock {
	fn version() -> u32 {
		70014
	}

	fn command() -> &'static str {
		"merkleblock"
	}

	fn deserialize_payload<T>(reader: &mut Reader<T>, _version: u32) -> MessageResult<Self>
	where
		T: io::Read,
	{
		let merkle_block = MerkleBlock {
			block_header: r#try!(reader.read()),
			total_transactions: r#try!(reader.read()),
			hashes: r#try!(reader.read_list()),
			flags: r#try!(reader.read()),
		};

		Ok(merkle_block)
	}

	fn serialize_payload(&self, stream: &mut Stream, _version: u32) -> MessageResult<()> {
		stream
			.append(&self.block_header)
			.append(&self.total_transactions)
			.append_list(&self.hashes)
			.append(&self.flags);
		Ok(())
	}
}