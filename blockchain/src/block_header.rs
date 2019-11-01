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
