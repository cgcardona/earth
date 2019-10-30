use clap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug)]
pub struct Config {}

pub fn parse_input(matches: &clap::ArgMatches) -> Result<Config, String> {
    println!("{:#?}", matches);
    Ok(Config {})
}
