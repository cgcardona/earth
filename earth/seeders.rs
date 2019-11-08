// TODO - Resolve these including IP address

pub fn mainnet_seeders() -> Vec<String> {
    vec![
        // String::from("seed.bitcoinabc.org:8333"),
        // String::from("seed.deadalnix.me:8333"),
        String::from("seed.bitcoinabc.org"),
        String::from("seed.deadalnix.me"),
    ]
}

pub fn testnet_seeders() -> Vec<String> {
    vec![
        // String::from("testnet-seed.bitcoinabc.org:18333"),
        // String::from("testnet-seed.deadalnix.me:18333"),
        String::from("testnet-seed.bitcoinabc.org"),
        String::from("testnet-seed.deadalnix.me"),
    ]
}
