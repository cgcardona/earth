use configuration::Configuration;
use std::{fs, path::PathBuf};

mod storage;

pub use storage::Storage;

pub struct DataBase {}

impl DataBase {
    pub fn init(c: &Configuration) {}

    /// create data_dir if it doesn't exist
    pub fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
        let p: PathBuf = [data_dir, sub].iter().collect();

        fs::create_dir_all(&p).expect("Failed to get app dir");

        p
    }
}
