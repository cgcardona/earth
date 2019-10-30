#[macro_use]
extern crate clap;
mod config;

pub use clap::{App, ArgMatches};
use config::parse_input;
use config::Config;

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    run();
    // if let Err(err) = run() {
    //     println!("{}", err);
    // }
}

fn run() {
    let command_line_options = load_yaml!("command_line_options.yml");
    let command_line_matches: ArgMatches = clap::App::from_yaml(command_line_options).get_matches();
    let configuration: Result<Config, String> = parse_input(&command_line_matches);
    println!("{:#?}", configuration);
}
