use crate::{MessageResult, Payload};
use ser::{Reader, Stream};
use std::io;

#[derive(Debug, PartialEq)]
pub struct SendCompact {
	pub first: bool,
	pub second: u64,
}

impl Payload for SendCompact {
	fn version() -> u32 {
		70014
	}

	fn command() -> &'static str {
		"sendcmpct"
	}

	fn deserialize_payload<T>(reader: &mut Reader<T>, _version: u32) -> MessageResult<Self>
	where
		T: io::Read,
	{
		let send_compact = SendCompact {
			first: r#try!(reader.read()),
			second: r#try!(reader.read()),
		};

		Ok(send_compact)
	}

	fn serialize_payload(&self, stream: &mut Stream, _version: u32) -> MessageResult<()> {
		stream.append(&self.first).append(&self.second);
		Ok(())
	}
}
