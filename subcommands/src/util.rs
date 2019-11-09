use configuration::Configuration;
use p2p::P2P;
use std::path::PathBuf;

pub const VERSION: u32 = 70_014;
pub const MIN: u32 = 70_001;

pub struct Util {}

impl Util {
    fn new() -> Self {
        Util {}
    }

    pub fn node_table_path(config: &Configuration) -> PathBuf {
        // create p2p directory
        let mut node_table_path: PathBuf = match config.data_dir {
            Some(ref data_dir) => P2P::create_p2p_dir(&data_dir, "p2p"),
            None => P2P::create_p2p_dir("data-dir", "p2p"),
        };

        node_table_path.push("nodes.csv");

        node_table_path
    }
}
