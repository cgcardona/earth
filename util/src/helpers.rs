use clap::App;
use serde_derive::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug)]
pub struct Config {
    db_cache: usize,
    data_dir: Option<String>,
}

pub const DEFAULT_DB_CACHE: usize = 512;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn parse(matches: &clap::ArgMatches) -> Result<Config, String> {
    let db_cache = match matches.value_of("db-cache") {
        Some(s) => s
            .parse()
            .map_err(|_| "Invalid cache size - should be number in MB".to_owned())?,
        None => DEFAULT_DB_CACHE,
    };

    let data_dir = match matches.value_of("data-dir") {
        Some(s) => Some(s.parse().map_err(|_| "Invalid data-dir".to_owned())?),
        None => None,
    };
    Ok(Config {
        db_cache: db_cache,
        data_dir: data_dir,
    })
}
