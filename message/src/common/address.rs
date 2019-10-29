use crate::bytes::Bytes;
use crate::common::{IpAddress, Port, Services};
use chain::{BlockHeader, ReadAndHash, ShortTransactionID};
use ser::deserialize;
use ser::{Deserializable, Error as ReaderError, Reader, Serializable, Stream};
use serde::{Deserialize, Serialize};
use std::{cmp, fmt, io};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NetAddress {
	pub services: Services,
	pub address: IpAddress,
	pub port: Port,
}

impl From<&'static str> for NetAddress {
	fn from(s: &'static str) -> Self {
		let bytes: Bytes = s.into();
		deserialize(bytes.as_ref()).unwrap()
	}
}

// TODO this is a placeholder Serializable impl
impl Serializable for NetAddress {
	fn serialize(&self, stream: &mut Stream) {}
}

// // TODO this is a placeholder Deserializable impl
impl Deserializable for NetAddress {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
	where
		T: io::Read,
	{
		let data = r#try!(reader.read::<NetAddress>());
		let header = NetAddress {
			services: data.services,
			address: data.address,
			port: data.port,
		};

		Ok(header)
	}
}

#[cfg(test)]
mod tests {
	use super::NetAddress;
	use common::Services;
	use ser::{deserialize, serialize};

	#[test]
	fn test_net_address_serialize() {
		let expected = vec![
			0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x0a, 0x00, 0x00, 0x01, 0x20, 0x8d,
		]
		.into();

		let address = NetAddress {
			services: Services::default().with_network(true),
			address: "::ffff:a00:1".into(),
			port: 8333.into(),
		};

		assert_eq!(serialize(&address), expected);
	}

	#[test]
	fn test_net_address_deserialize() {
		let bytes = vec![
			0x01u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x0a, 0x00, 0x00, 0x01, 0x20, 0x8d,
		];

		let expected = NetAddress {
			services: Services::default().with_network(true),
			address: "::ffff:a00:1".into(),
			port: 8333.into(),
		};

		assert_eq!(expected, deserialize(&bytes as &[u8]).unwrap());
	}

	#[test]
	fn test_net_address_from_static_str() {
		let expected = NetAddress {
			services: Services::default().with_network(true),
			address: "::ffff:a00:1".into(),
			port: 8333.into(),
		};
		let s = "010000000000000000000000000000000000ffff0a000001208d";
		assert_eq!(expected, s.into());
	}
}
