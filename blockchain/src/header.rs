use serde::{Deserialize, Serialize};
use std::fmt;

/// BlockHeader struct
#[derive(Serialize, Deserialize)]
pub struct Header {
    pub version: u32,
    pub time: usize,
    pub bits: String,
    pub nonce: u64,
    pub prev_hash: String,
    pub merkle_hash: String,
}

/// impl Header
impl Header {
    pub fn new(
        version: u32,
        time: usize,
        bits: String,
        nonce: u64,
        prev_hash: String,
        merkle_hash: String,
    ) -> Self {
        Header {
            version: version,
            time: time,
            bits: bits,
            nonce: nonce,
            prev_hash: prev_hash,
            merkle_hash: merkle_hash,
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: Default::default(),
            time: Default::default(),
            bits: Default::default(),
            nonce: Default::default(),
            prev_hash: Default::default(),
            merkle_hash: Default::default(),
        }
    }
}

/// impl from for Header
impl From<&'static str> for Header {
    fn from(s: &'static str) -> Self {
        Header::new(1, 1, String::from(s), 1, String::from(s), String::from(s))
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Header")
            .field("version", &self.version)
            .field("prev_hash", &self.prev_hash)
            .field("merkle_hash", &self.merkle_hash)
            .field("time", &self.time)
            .field("bits", &self.bits)
            .field("nonce", &self.nonce)
            .finish()
    }
}
