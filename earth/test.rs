#[cfg(test)]
mod tests {
    // use blockchain::Block;
    // use database::Storage;
    // use mock_data::block_mock_data;
    // #[test]

    // fn insert_block() {
    //     let s: Storage = Storage {};

    // let f: Block = Block::from("foooooobar");
    // println!("{:#?}", f);
    // assert_eq!(1, 1);
    // let b0: Block = mock_data::block_mock_data();

    // let key: &str = "foo";
    // let serialized = serde_json::to_string(&b0).unwrap();

    // assert!(s::write(key, serialized).is_ok());
    // }

    // #[test]
    // fn read_block() {
    //     let key: &str = "foo";
    //     match database::read(key) {
    //         Ok(Some(value)) => match value.to_utf8() {
    //             Some(v) => println!("Reading key: {} and value: {} from rocksdb", key, v),
    //             None => println!("did not read valid utf-8 out of the db"),
    //         },
    //         Ok(None) => panic!("value not present!"),
    //         Err(e) => println!("error retrieving value: {}", e),
    //     }

    //     // let store = BlockChainDatabase::open(MemoryDatabase::default());
    //     // let b0: IndexedBlock = test_data::block_h0().into();
    //     // let b1: IndexedBlock = test_data::block_h1().into();
    //     // let b2: IndexedBlock = test_data::block_h2().into();

    //     // store.insert(b0.clone()).unwrap();
    //     // store.insert(b1.clone()).unwrap();
    //     // store.insert(b2.clone()).unwrap();

    //     // assert_eq!(0, store.best_block().number);
    //     // assert!(store.best_block().hash.is_zero());

    //     // store.canonize(b0.hash()).unwrap();
    //     // assert_eq!(0, store.best_block().number);
    //     // assert_eq!(b0.hash(), &store.best_block().hash);

    //     // store.canonize(b1.hash()).unwrap();
    //     // assert_eq!(1, store.best_block().number);
    //     // assert_eq!(b1.hash(), &store.best_block().hash);

    //     // store.canonize(b2.hash()).unwrap();
    //     // assert_eq!(2, store.best_block().number);
    //     // assert_eq!(b2.hash(), &store.best_block().hash);

    //     // let decanonized = store.decanonize().unwrap();
    //     // assert_eq!(b2.hash(), &decanonized);
    //     // assert_eq!(1, store.best_block().number);
    //     // assert_eq!(b1.hash(), &store.best_block().hash);

    //     // assert_eq!(b0.hash(), &store.block_hash(0).unwrap());
    //     // assert_eq!(b1.hash(), &store.block_hash(1).unwrap());
    //     // assert!(store.block_hash(2).is_none());

    //     // assert_eq!(0, store.block_number(b0.hash()).unwrap());
    //     // assert_eq!(1, store.block_number(b1.hash()).unwrap());
    //     // assert!(store.block_number(b2.hash()).is_none());
    // }

    // #[test]
    // fn delete_block() {
    //     let key: &str = "foo";
    //     assert!(database::delete(key).is_ok());
    // }
}
