use core::result::Result;
use rocksdb::DB;
use std::path::PathBuf;

pub fn write(k: &str, v: String) -> Result<(), String> {
    // TODO - Remove hard coded paths
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.put(k, v);
    Ok(())
}

pub fn read(
    k: &str,
) -> std::result::Result<std::option::Option<rocksdb::DBVector>, rocksdb::Error> {
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.get(k)
}

pub fn delete(k: &str) -> Result<(), String> {
    let p: PathBuf = ["data-dir", "db"].iter().collect();
    let db: rocksdb::DB = DB::open_default(&p).unwrap();
    db.delete(k);
    Ok(())
}
