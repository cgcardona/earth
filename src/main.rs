#[macro_use]
extern crate clap;
// extern crate log;
extern crate app_dirs;
// extern crate env_logger;
// extern crate libc;

extern crate chain;
// extern crate db;
// extern crate import;
// extern crate keys;
// extern crate logs;
// extern crate message;
extern crate network;
extern crate p2p;
// extern crate primitives;
extern crate rpc as ethcore_rpc;
// extern crate script;
extern crate storage;
// extern crate sync;
// extern crate verification;

// mod commands;
mod config;
mod rpc;
mod rpc_apis;
mod seednodes;
mod util;

use app_dirs::AppInfo;
pub use clap::App;
use commands::start;
use config::parse;

pub const USER_AGENT: &'static str = "earth";
pub const REGTEST_USER_AGENT: &'static str = "/EARTH:0.0.1/";

pub const APP_INFO: AppInfo = AppInfo {
    name: "earth",
    author: "EARTH",
};

pub const PROTOCOL_VERSION: u32 = 70_014;
pub const PROTOCOL_MINIMUM: u32 = 70_001;
pub const LOG_INFO: &'static str = "sync=info";

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    run();
    // if let Err(err) = run() {
    //     println!("{}", err);
    // }
}

fn run() {
    let yml = load_yaml!("cli.yml");
    let matches: clap::ArgMatches<'_> = clap::App::from_yaml(yml).get_matches();
    let cfg = parse(&matches);
    println!("{:#?}", cfg);
}
