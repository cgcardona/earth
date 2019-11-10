#[macro_use]
extern crate clap;

use clap::ArgMatches;
use configuration::Configuration;
use subcommands::{import, rollback, start};

/// main function
fn main() {
    better_panic::install();

    // get command line options from yaml
    let cli_options = load_yaml!("command_line_options.yml");

    // match command line input to options
    let cli_matches: ArgMatches = clap::App::from_yaml(cli_options).get_matches();

    // parse configuration from command line
    let config: Configuration = Configuration::parse_input(&cli_matches).unwrap();

    // detect subcommand
    match cli_matches.subcommand() {
        ("import", Some(matches)) => {
            // `import` subcommand
            import(config, matches);
        }
        ("rollback", Some(matches)) => {
            // `rollback` subcommand
            rollback(config, matches);
        }
        _ => {
            // run `start` subcommand by default
            start(config);
        }
    }
}
