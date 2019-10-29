pub fn mainnet_seednodes() -> Vec<&'static str> {
    vec![
        "seed.bitcoinabc.org:8333",
        "seed-abc.bitcoinforks.org:8333",
        "seed.bitprim.org:8333",
        "seed.deadalnix.me:8333",
        "seeder.criptolayer.net:8333",
        "seed.bchd.cash:8333",
    ]
}

pub fn testnet_seednodes() -> Vec<&'static str> {
    vec![
        "testnet-seed.bitcoinabc.org:18333",
        "testnet-seed-abc.bitcoinforks.org:18333",
        "testnet-seed.bitprim.org:18333",
        "testnet-seed.deadalnix.me:18333",
        "testnet-seeder.criptolayer.net:18333",
    ]
}
