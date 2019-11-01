#[macro_use]
extern crate clap;
extern crate database;
extern crate mock_data;
extern crate network;

mod configuration;
mod seeders;
mod subcommands;

use clap::ArgMatches;
pub use configuration::{parse_input, Configuration};
pub use seeders::{mainnet_seeders, testnet_seeders};
use subcommands::{import, rollback, start};

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    run();
}

fn run() {
    let command_line_options = load_yaml!("command_line_options.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(command_line_options).get_matches();

    // detect configuration from command line
    let configuration: Result<Configuration, String> =
        configuration::parse_input(&command_line_matches);

    println!("configuration: {:#?}", &configuration);

    let config: Configuration = configuration.unwrap();

    // detect subcommands (import)
    match command_line_matches.subcommand() {
        ("import", Some(matches)) => {
            import(&config, matches);
        }
        ("rollback", Some(matches)) => {
            rollback(&config, matches);
        }
        _ => {
            start(config);
        }
    }
}
