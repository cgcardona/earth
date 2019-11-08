#[macro_use]
extern crate clap;
extern crate database;
extern crate mock_data;
extern crate network;

mod test;

use clap::ArgMatches;
use configuration::Configuration;
use subcommands::{import, rollback, start};

/// main function
fn main() {
    better_panic::install();

    run();
}

/// runner
fn run() {
    let command_line_options = load_yaml!("command_line_options.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(command_line_options).get_matches();

    // detect configuration from command line
    let config: Configuration = configuration::parse_input(&command_line_matches).unwrap();
    // println!("configuration: {:#?}", &configuration);

    // detect subcommand
    match command_line_matches.subcommand() {
        ("import", Some(matches)) => {
            import(&config, matches);
        }
        ("rollback", Some(matches)) => {
            rollback(&config, matches);
        }
        _ => {
            // run `start` subcommand by default
            start(config);
        }
    }
}
