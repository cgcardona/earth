use crate::common::Command;
use crate::hash::H32;
use crate::Error;
use crypto::checksum;
use network::Magic;
use ser::{Reader, Serializable, Stream};

#[derive(Debug, PartialEq)]
pub struct MessageHeader {
	pub magic: Magic,
	pub command: Command,
	pub len: u32,
	pub checksum: H32,
}

impl MessageHeader {
	pub fn for_data(magic: Magic, command: Command, data: &[u8]) -> Self {
		MessageHeader {
			magic: magic,
			command: command,
			len: data.len() as u32,
			checksum: checksum(data),
		}
	}
}

impl MessageHeader {
	pub fn deserialize(data: &[u8], expected: Magic) -> Result<Self, Error> {
		if data.len() != 24 {
			return Err(Error::Deserialize);
		}

		let mut reader = Reader::new(data);
		let magic: u32 = r#try!(reader.read());
		let magic = Magic::from(magic);
		if expected != magic {
			return Err(Error::InvalidMagic);
		}

		let header = MessageHeader {
			magic: magic,
			command: r#try!(reader.read()),
			len: r#try!(reader.read()),
			checksum: r#try!(reader.read()),
		};

		Ok(header)
	}
}

impl Serializable for MessageHeader {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&self.magic)
			.append(&self.command)
			.append(&self.len)
			.append(&self.checksum);
	}
}

#[cfg(test)]
mod tests {
	use super::MessageHeader;
	use bytes::Bytes;
	use network::{ConsensusFork, Network};
	use ser::serialize;

	#[test]
	fn test_message_header_serialization() {
		let expected = "f9beb4d96164647200000000000000001f000000ed52399b".into();
		let header = MessageHeader {
			magic: Network::Mainnet.magic(&ConsensusFork::BitcoinCore),
			command: "addr".into(),
			len: 0x1f,
			checksum: "ed52399b".into(),
		};

		assert_eq!(serialize(&header), expected);
	}

	#[test]
	fn test_message_header_deserialization() {
		let raw: Bytes = "f9beb4d96164647200000000000000001f000000ed52399b".into();
		let expected = MessageHeader {
			magic: Network::Mainnet.magic(&ConsensusFork::BitcoinCore),
			command: "addr".into(),
			len: 0x1f,
			checksum: "ed52399b".into(),
		};

		assert_eq!(
			expected,
			MessageHeader::deserialize(&raw, Network::Mainnet.magic(&ConsensusFork::BitcoinCore))
				.unwrap()
		);
	}
}
