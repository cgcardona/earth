use core::result::Result;
use rocksdb::DB;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Storage {
    pub data_dir: String,
}

impl Storage {
    pub fn new(data_dir: String) -> Self {
        Storage { data_dir: data_dir }
    }

    pub fn write(&self, k: &str, v: String) -> Result<(), String> {
        let p: PathBuf = [&self.data_dir].iter().collect();
        let db: rocksdb::DB = DB::open_default(&p).unwrap();
        db.put(k, v);
        Ok(())
    }

    pub fn read(
        &self,
        k: &str,
    ) -> std::result::Result<std::option::Option<rocksdb::DBVector>, rocksdb::Error> {
        let p: PathBuf = [&self.data_dir].iter().collect();
        let db: rocksdb::DB = DB::open_default(&p).unwrap();
        match db.get(k) {
            Ok(Some(value)) => println!("retrieved value {:#?}", value.to_utf8().unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {:#?}", e),
        }

        db.get(k)
    }

    pub fn delete(&self, k: &str) -> Result<(), String> {
        let p: PathBuf = [&self.data_dir].iter().collect();
        let db: rocksdb::DB = DB::open_default(&p).unwrap();
        db.delete(k);
        Ok(())
    }
}
