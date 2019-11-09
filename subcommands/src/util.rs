use configuration::Configuration;
use p2p::Config;
use p2p::{dns_lookup, P2P};
use p2p::{LocalSyncNode, LocalSyncNodeRef};
use std::{fs, path::PathBuf};
use tokio_core::reactor::{Core, Handle};

pub const VERSION: u32 = 70_014;
pub const MIN: u32 = 70_001;

pub struct Util {}

impl Util {
    fn new() -> Self {
        Util {}
    }

    /// Start database setup
    pub fn start_db(c: &Configuration) {
        // create db directory
        match c.data_dir {
            Some(ref data_dir) => Util::create_data_dir(&data_dir, "db"),
            None => Util::create_data_dir("data-dir", "db"),
        };

        // let data_dir: String = match c.data_dir {
        //     Some(ref data_dir) => String::from(data_dir),
        //     None => String::from("data-dir"),
        // };
    }

    pub fn initialize_database(c: &Configuration) {}

    pub fn node_table_path(config: &Configuration) -> PathBuf {
        // create p2p directory
        let mut node_table_path: PathBuf = match config.data_dir {
            Some(ref data_dir) => Util::create_p2p_dir(&data_dir, "p2p"),
            None => Util::create_p2p_dir("data-dir", "p2p"),
        };

        node_table_path.push("nodes.csv");

        node_table_path
    }

    /// create data_dir if it doesn't exist
    fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
        let p: PathBuf = [data_dir, sub].iter().collect();

        fs::create_dir_all(&p).expect("Failed to get app dir");

        p
    }

    /// create p2p directory if it doesn't exist
    fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
        let p: PathBuf = [p2p, sub].iter().collect();

        fs::create_dir_all(&p).expect("Failed to get p2p dir");

        p
    }
}
