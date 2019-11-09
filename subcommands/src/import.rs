use clap::ArgMatches;
use configuration::Configuration;

/// imports blockchain data
pub fn import(c: &Configuration, m: &ArgMatches) -> Result<(), String> {
    let g: &str = c.network.genesis();
    let i: &str = m.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", m, g, i);
    Ok(())
}
