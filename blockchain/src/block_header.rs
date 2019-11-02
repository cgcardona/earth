use serde::{Deserialize, Serialize};

/// BlockHeader struct
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle_hash: String,
}

impl From<&'static str> for BlockHeader {
    fn from(s: &'static str) -> Self {
        BlockHeader {
            version: 1,
            time: 1,
            bits: 1,
            nonce: 1,
            prev_hash: String::from(s),
            merkle_hash: String::from(s),
        }
    }
}
