use crate::seeders::{mainnet_seeders, testnet_seeders};
use crate::Configuration;
use mock_data::{block_mock_data, Block};
use rocksdb::DB;
use std::{fs, path::PathBuf};

/// imports blockchain data
pub fn import(c: &Configuration, matches: &clap::ArgMatches) {
    let genesis: String = c.network.genesis();
    let import_path: &str = matches.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", matches, genesis, import_path);
}

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, matches: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", c, matches);
}

/// start EARTH client with command line arguments
pub fn start(c: Configuration) {
    start_db(&c);
    start_p2p(&c);
}

/// Start database setup
fn start_db(c: &Configuration) {
    // create db directory
    match c.data_dir {
        Some(ref data_dir) => create_data_dir(&data_dir, "db"),
        None => create_data_dir("data-dir", "db"),
    };
}

/// Start p2p connections
fn start_p2p(c: &Configuration) {
    // create p2p directory
    match c.data_dir {
        Some(ref data_dir) => create_p2p_dir(&data_dir, "p2p"),
        None => create_p2p_dir("data-dir", "p2p"),
    };
}

/// create data_dir if it doesn't exist
fn create_data_dir(data_dir: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [data_dir, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get app dir");

    let db: rocksdb::DB = DB::open_default(&p).unwrap();

    let b0: Block = mock_data::block_mock_data();

    let key: &str = "foo";
    let serialized = serde_json::to_string(&b0).unwrap();

    assert!(db.put(key, serialized).is_ok());

    match db.get(key) {
        Ok(Some(value)) => match value.to_utf8() {
            Some(v) => println!("Reading key: {} and value: {} from rocksdb", key, v),
            None => println!("did not read valid utf-8 out of the db"),
        },
        Ok(None) => panic!("value not present!"),
        Err(e) => println!("error retrieving value: {}", e),
    }

    assert!(db.delete(key).is_ok());

    p
}

/// create p2p directory if it doesn't exist
fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [p2p, sub].iter().collect();
    fs::create_dir_all(&p).expect("Failed to get p2p dir");
    p
}
