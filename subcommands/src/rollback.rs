use clap::ArgMatches;
use configuration::Configuration;

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, m: &ArgMatches) -> Result<(), String> {
    println!("ROLLBACK: {:#?}, {:#?}", c, m);
    Ok(())
}
