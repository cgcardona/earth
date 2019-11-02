use serde::{Deserialize, Serialize};
use std::fmt;

/// BlockHeader struct
#[derive(Serialize, Deserialize)]
pub struct Header {
    pub version: u32,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle_hash: String,
}

/// impl from for Header
impl From<&'static str> for Header {
    fn from(s: &'static str) -> Self {
        Header::new(1, 1, 1, 1, String::from(s), String::from(s))
    }
}

/// impl Header
impl Header {
    pub fn new(
        version: u32,
        time: u32,
        bits: u32,
        nonce: u32,
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
