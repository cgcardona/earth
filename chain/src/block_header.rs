use std::time::SystemTime;

#[derive(Debug, Hash)]
pub struct BlockHeader {
    version: u32,
    bits: u32,
    nonce: u32,
    merkle_root_hash: u64,
    prev_header_hash: u64,
    time: SystemTime,
}

impl BlockHeader {
    pub fn new(
        version: u32,
        bits: u32,
        nonce: u32,
        merkle_root_hash: u64,
        prev_header_hash: u64,
        time: SystemTime,
    ) -> BlockHeader {
        BlockHeader {
            version: version,
            bits: bits,
            nonce: nonce,
            merkle_root_hash: merkle_root_hash,
            prev_header_hash: prev_header_hash,
            time: time,
        }
    }

    pub fn version(&self) -> &u32 {
        &self.version
    }

    pub fn bits(&self) -> &u32 {
        &self.bits
    }

    pub fn nonce(&self) -> &u32 {
        &self.nonce
    }

    pub fn merkle_root_hash(&self) -> &u64 {
        &self.merkle_root_hash
    }

    pub fn prev_header_hash(&self) -> &u64 {
        &self.prev_header_hash
    }

    pub fn time(&self) -> &SystemTime {
        &self.time
    }
}
