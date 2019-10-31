#[macro_use]
extern crate clap;
extern crate network;

mod configuration;
mod subcommands;

use clap::ArgMatches;
pub use configuration::{parse_input, Configuration};
use subcommands::import;

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
    println!("Configuration: {:#?}", &configuration);

    let config: Configuration = configuration.unwrap();

    // detect subcommands (import)
    match command_line_matches.subcommand() {
        ("import", Some(matches)) => {
            import(&config, matches);
        }
        _ => {}
    }
}
