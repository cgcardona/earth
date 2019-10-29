use crate::{MessageResult, Payload};
use ser::{Reader, Stream};
use std::io;

#[derive(Debug, PartialEq)]
pub struct MemPool;

impl Payload for MemPool {
	fn version() -> u32 {
		60002
	}

	fn command() -> &'static str {
		"mempool"
	}

	fn deserialize_payload<T>(_reader: &mut Reader<T>, _version: u32) -> MessageResult<Self>
	where
		T: io::Read,
	{
		Ok(MemPool)
	}

	fn serialize_payload(&self, _stream: &mut Stream, _version: u32) -> MessageResult<()> {
		Ok(())
	}
}
