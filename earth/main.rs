#[macro_use]
extern crate clap;
extern crate network;

mod config;

use clap::ArgMatches;
use config::{parse_input, Config};

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    run();
}

fn run() {
    let command_line_options = load_yaml!("command_line_options.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(command_line_options).get_matches();
    let configuration: Result<Config, String> = parse_input(&command_line_matches);
    println!("{:#?}", configuration);
}
