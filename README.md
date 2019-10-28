# EARTH

```
EARTH 0.0.1
EARTH <https://www.earth.engineering>
EARTH client

USAGE:
    EARTH [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --help          Prints help information
        --no-jsonrpc    Disable the JSON-RPC API server.
    -q, --quiet         Do not show any synchronization information in the console.
        --regtest       Use a private network for regression tests.
        --testnet       Use the test network (Testnet3).
    -V, --version       Prints version information

OPTIONS:
        --blocknotify <COMMAND>            Execute COMMAND when the best block changes (%s in COMMAND is replaced by the
                                           block hash).
    -c, --connect <IP>                     Connect only to the specified node.
    -d, --data-dir <PATH>                  Specify the database and configuration directory PATH.
        --db-cache <SIZE>                  Sets the database cache size.
    -h, --host <HOST>                      Listen for connections on HOST.
        --jsonrpc-apis <APIS>              Specify the APIs available through the JSONRPC interface. APIS is a comma-
                                           delimited list of API names. Available APIs are blockchain, network,
                                           miner, raw.
        --jsonrpc-cors <URL>               Specify CORS header for JSON-RPC API responses.
        --jsonrpc-hosts <HOSTS>            List of allowed Host header values.
        --jsonrpc-interface <INTERFACE>    The hostname portion of the JSONRPC API server.
        --jsonrpc-port <PORT>              Specify the PORT for the JSONRPC API server.
        --only-net <NET>                   Only connect to nodes in network version <NET> (ipv4 or ipv6).
        --port <PORT>                      Listen for connections on PORT.
    -s, --seednode <IP>                    Connect to a seed-node to retrieve peer addresses, and disconnect.
        --verification-edge <BLOCK>        Non-default verification-level is applied until a block with given hash is
                                           met.
        --verification-level <LEVEL>       Sets the Blocks verification level to full (default), header (scripts are not
                                           verified), or none (no verification at all).

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    import      Import blocks from a Bitcoin Core database.
    rollback    Rollback the database to given canonical-chain block.
```

```rust
    start();
    use chain::{Block, BlockHeader, Transaction, Vin, Vout};
    use std::time::SystemTime;
    use util::calculate_hash;

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
Start with user agent: /EARTH:0.0.1/
config p2p
config db
sync w/ peers
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
