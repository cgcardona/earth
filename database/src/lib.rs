extern crate rocksdb;

mod storage;

pub use storage::{delete, read, write};
