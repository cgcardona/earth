#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    pub fn port(&self) -> u16 {
        match *self {
            Network::Mainnet => 8332,
            Network::Testnet => 18332,
            Network::Regtest => 18443,
        }
    }
}
