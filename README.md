# EARTH

Adventures in crypto

```rs
    let version: u32 = 1;
    let bits: u32 = 1;
    let nonce: u32 = 1;
    let merkle_root_hash: u64 = 1;
    let prev_header_hash: u64 = 1;
    let time: SystemTime = SystemTime::now();

    let prev_block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );

    let prev_header_hash: u64 = calculate_hash(&prev_block_header);
    let merkle_root_hash: u64 = calculate_hash(&prev_block_header);
    let block_header: BlockHeader = BlockHeader::new(
        version,
        bits,
        nonce,
        merkle_root_hash,
        prev_header_hash,
        time,
    );

    let txid: String =
        String::from("fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33");
    let version: u32 = 1;
    let locktime: u32 = 1;
    let vin: Vec<Vin> = vec![];
    let vout: Vec<Vout> = vec![];
    let blockhash: String =
        String::from("00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09");
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
        merkle_root_hash: 13884905648120091660,
        prev_header_hash: 13884905648120091660,
        time: SystemTime {
            tv_sec: 1572242703,
            tv_nsec: 594824000,
        },
    },
    transactions: [
        Transaction {
            txid: "fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33",
            version: 1,
            locktime: 1,
            vin: [],
            vout: [],
            blockhash: "00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09",
            blockheight: 1,
            confirmations: 1,
            time: SystemTime {
                tv_sec: 1572242703,
                tv_nsec: 594879000,
            },
            blocktime: SystemTime {
                tv_sec: 1572242703,
                tv_nsec: 594879000,
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
    merkle_root_hash: 13884905648120091660,
    prev_header_hash: 13884905648120091660,
    time: SystemTime {
        tv_sec: 1572242703,
        tv_nsec: 594824000,
    },
}!
----------
[
    Transaction {
        txid: "fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33",
        version: 1,
        locktime: 1,
        vin: [],
        vout: [],
        blockhash: "00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09",
        blockheight: 1,
        confirmations: 1,
        time: SystemTime {
            tv_sec: 1572242703,
            tv_nsec: 594879000,
        },
        blocktime: SystemTime {
            tv_sec: 1572242703,
            tv_nsec: 594879000,
        },
        isCoinBase: false,
        valueOut: 1,
        size: 1,
    },
]!
```
