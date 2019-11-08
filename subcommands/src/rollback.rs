use clap;
use configuration::Configuration;

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, m: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", c, m);
}
