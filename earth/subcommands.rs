use crate::Configuration;
use blockchain::Block;
use database::Storage;
use mock_data::block_mock_data;
use std::{fs, path::PathBuf};

// use crate::seeders::{mainnet_seeders, testnet_seeders};
// use blockchain::Block;
// use database::*;
// use mock_data::block_mock_data;

/// imports blockchain data
pub fn import(c: &Configuration, m: &clap::ArgMatches) {
    let g: &str = c.network.genesis();
    let i: &str = m.value_of("PATH").unwrap();
    println!("IMPORT: {:#?}, {}, {}", m, g, i);
}

/// Rollback the database to block hash or number
pub fn rollback(c: &Configuration, m: &clap::ArgMatches) {
    println!("ROLLBACK: {:#?}, {:#?}", c, m);
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

    let data_dir: String = match c.data_dir {
        Some(ref data_dir) => String::from(data_dir),
        None => String::from("data-dir"),
    };

    let s: Storage = Storage::new(data_dir);

    let b: Block = Block::new(None, None);
    println!("Block: {:#?}", b);

    // for x in 0..1 {
    //     let b: Block = mock_data::block_mock_data(x);
    //     let key: &str = &format!("{}{}", "foo", x);
    //     let serialized0 = serde_json::to_string(&b).unwrap();
    //     assert!(s.write(key, serialized0).is_ok());
    //     assert!(s.read(key).is_ok());
    //     assert!(s.delete(key).is_ok());
    // }

    // let b0: Block = mock_data::block_mock_data();
    // let b1: Block = mock_data::block_mock_data();
    // let b2: Block = mock_data::block_mock_data();
    // let b3: Block = mock_data::block_mock_data();
    // let b4: Block = mock_data::block_mock_data();

    // let key0: &str = "foo0";
    // let key1: &str = "foo1";
    // let key2: &str = "foo2";
    // let key3: &str = "foo3";
    // let key4: &str = "foo4";

    // let serialized0 = serde_json::to_string(&b0).unwrap();
    // let serialized1 = serde_json::to_string(&b1).unwrap();
    // let serialized2 = serde_json::to_string(&b2).unwrap();
    // let serialized3 = serde_json::to_string(&b3).unwrap();
    // let serialized4 = serde_json::to_string(&b4).unwrap();

    // assert!(s.write(key0, serialized0).is_ok());
    // assert!(s.write(key1, serialized1).is_ok());
    // assert!(s.write(key2, serialized2).is_ok());
    // assert!(s.write(key3, serialized3).is_ok());
    // assert!(s.write(key4, serialized4).is_ok());
    // assert!(s.read(key0).is_ok());
    // assert!(s.read(key1).is_ok());
    // assert!(s.read(key2).is_ok());
    // assert!(s.read(key3).is_ok());
    // assert!(s.read(key4).is_ok());
    // assert!(s.delete(key0).is_ok());
    // assert!(s.delete(key1).is_ok());
    // assert!(s.delete(key2).is_ok());
    // assert!(s.delete(key3).is_ok());
    // assert!(s.delete(key4).is_ok());
    // println!("FOO {:#?}", z);
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

    p
}

/// create p2p directory if it doesn't exist
fn create_p2p_dir(p2p: &str, sub: &str) -> PathBuf {
    let p: PathBuf = [p2p, sub].iter().collect();

    fs::create_dir_all(&p).expect("Failed to get p2p dir");

    p
}
