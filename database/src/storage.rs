use core::result::Result;
use rocksdb::DB;
use std::path::PathBuf;

pub fn write(key: &str, value: String) -> Result<(), String> {
    // TODO - Remove hard coded paths
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.put(key, value);
    Ok(())
}

pub fn read(
    key: &str,
) -> std::result::Result<std::option::Option<rocksdb::DBVector>, rocksdb::Error> {
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.get(key)
}

pub fn delete(key: &str) -> Result<(), String> {
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.delete(key);
    Ok(())
}
