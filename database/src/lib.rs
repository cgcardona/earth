use configuration::Configuration;

mod storage;

pub use storage::Storage;

pub struct DataBase {}

impl DataBase {
    pub fn initialize_database(c: &Configuration) {}
}
