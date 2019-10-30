#[macro_use]
extern crate clap;
extern crate network;

mod configuration;

use clap::ArgMatches;
use configuration::{parse_input, Configuration};

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    run();
}

fn run() {
    let command_line_options = load_yaml!("command_line_options.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(command_line_options).get_matches();
    let configuration: Result<Configuration, String> = parse_input(&command_line_matches);
    println!("{:#?}", configuration);
}
