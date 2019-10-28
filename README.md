# EARTH

Adventures in crypto

```rs
// create new BlockHeader
let version: u32 = 1;
let bits: u32 = 1;
let nonce: u32 = 1;
let merkle_root_hash: u32 = 1;
let prev_header_hash: u32 = 1;
let time: SystemTime = SystemTime::now();
let block_header: BlockHeader = BlockHeader::new(
    version,
    bits,
    nonce,
    merkle_root_hash,
    prev_header_hash,
    time,
);

// create new Transaction
let txid: String = String::from("hello world");
let version: u32 = 1;
let locktime: u32 = 1;
let vin: Vec<Vin> = vec![];
let vout: Vec<Vout> = vec![];
let blockhash: String = String::from("hello world");
let blockheight: u32 = 1;
let confirmations: u32 = 1;
let time: SystemTime = SystemTime::now();
let blocktime: SystemTime = SystemTime::now();
let isCoinBase: bool = false;
let valueOut: u32 = 1;
let size: u32 = 1;
    let transcation: Transaction = Transaction::new(
    txid,
    version,
    locktime,
    vin,
    vout,
    blockhash,
    blockheight,
    confirmations,
    time,
    blocktime,
    isCoinBase,
    valueOut,
    size,
);

// create new Block
let block: Block = Block::new(block_header, vec![transcation]);
println!("{:#?}!", block);
println!("----------");
println!("{:#?}!", block.block_header());
println!("----------");
println!("{:#?}!", block.transactions());
```

That logs the following

```
Block {
    block_header: BlockHeader {
        version: 1,
        bits: 1,
        nonce: 1,
        merkle_root_hash: 1,
        prev_header_hash: 1,
        time: SystemTime {
            tv_sec: 1572241072,
            tv_nsec: 395945000,
        },
    },
    transactions: [
        Transaction {
            txid: "hello world",
            version: 1,
            locktime: 1,
            vin: [],
            vout: [],
            blockhash: "hello world",
            blockheight: 1,
            confirmations: 1,
            time: SystemTime {
                tv_sec: 1572241072,
                tv_nsec: 395966000,
            },
            blocktime: SystemTime {
                tv_sec: 1572241072,
                tv_nsec: 395966000,
            },
            isCoinBase: false,
            valueOut: 1,
            size: 1,
        },
    ],
}!
----------
BlockHeader {
    version: 1,
    bits: 1,
    nonce: 1,
    merkle_root_hash: 1,
    prev_header_hash: 1,
    time: SystemTime {
        tv_sec: 1572241072,
        tv_nsec: 395945000,
    },
}!
----------
[
    Transaction {
        txid: "hello world",
        version: 1,
        locktime: 1,
        vin: [],
        vout: [],
        blockhash: "hello world",
        blockheight: 1,
        confirmations: 1,
        time: SystemTime {
            tv_sec: 1572241072,
            tv_nsec: 395966000,
        },
        blocktime: SystemTime {
            tv_sec: 1572241072,
            tv_nsec: 395966000,
        },
        isCoinBase: false,
        valueOut: 1,
        size: 1,
    },
]!
```
