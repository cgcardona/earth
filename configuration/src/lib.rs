mod configuration;
mod seeders;

pub use configuration::{parse_input, Configuration};
pub use seeders::{mainnet_seeders, testnet_seeders};
