pub fn mainnet_seeders() -> Vec<String> {
    vec![
        String::from("seed.bitcoinabc.org:8333"),
        String::from("seed-abc.bitcoinforks.org:8333"),
        String::from("seed.bitprim.org:8333"),
        String::from("seed.deadalnix.me:8333"),
        String::from("seeder.criptolayer.net:8333"),
    ]
}

pub fn testnet_seeders() -> Vec<String> {
    vec![
        String::from("testnet-seed.bitcoinabc.org:18333"),
        String::from("testnet-seed-abc.bitcoinforks.org:18333"),
        String::from("testnet-seed.bitprim.org:18333"),
        String::from("testnet-seed.deadalnix.me:18333"),
        String::from("testnet-seeder.criptolayer.net:18333"),
    ]
}
