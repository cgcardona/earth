use serde_derive::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
    Unitest,
    Other(u32),
}

impl Network {
    pub fn port(&self) -> u16 {
        match *self {
            Network::Mainnet | Network::Other(_) => 8333,
            Network::Testnet => 18333,
            Network::Regtest | Network::Unitest => 18444,
        }
    }
}
