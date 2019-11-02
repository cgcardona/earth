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

impl From<&'static str> for Header {
    fn from(s: &'static str) -> Self {
        Header {
            version: 1,
            time: 1,
            bits: 1,
            nonce: 1,
            prev_hash: String::from(s),
            merkle_hash: String::from(s),
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
