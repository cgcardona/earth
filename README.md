# EARTH

```
EARTH 0.0.1
EARTH <https://www.earth.engineering>
EARTH client

USAGE:
    earth [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -q, --quiet      Do not show any synchronization information in the console.
        --regtest    Use a private network for regression tests.
        --testnet    Use the test network
    -V, --version    Prints version information

OPTIONS:
        --blocknotify <COMMAND>         Execute COMMAND when the best block changes (%s in COMMAND is replaced by the
                                        block hash).
    -c, --connect <IP>                  Connect only to the specified node.
    -d, --data-dir <PATH>               Specify the database and configuration directory PATH.
        --db-cache <SIZE>               Sets the database cache size.
    -h, --host <HOST>                   Listen for connections on HOST.
        --only-net <NET>                Only connect to nodes in network version <NET> (ipv4 or ipv6).
        --port <PORT>                   Listen for connections on PORT.
    -s, --seednode <IP>                 Connect to a seed-node to retrieve peer addresses, and disconnect.
        --verification-edge <BLOCK>     Non-default verification-level is applied until a block with given hash is met.
        --verification-level <LEVEL>    Sets the Blocks verification level to full (default), header (scripts are not
                                        verified), or none (no verification at all).

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    import      Import blocks from a database.
    rollback    Rollback the database to given canonical-chain block.
```

## Usage

**WARNING** `earth --data-dir foobar` will create `./foobar/db/` and `./foobar/p2p/` directories in the root directory from which you ran the command. It will create recursive directories: `earth --data-dir foo/bar`

If you don't pass in a `--data-dir` flag then `./data-dir/db/` and `./data-dir/p2p/` directories will be created in the root directory from which you ran the command.

```
./target/debug/earth --data-dir r/e/w/t/r/e/w
configuration: Ok(
    Configuration {
        network: Mainnet,
        data_dir: Some(
            "r/e/w/t/r/e/w",
        ),
        port: 8332,
        db_cache: 512,
        user_agent: "/EARTH:0.0.1/",
        quiet: false,
        seeders: [
            "seed.bitcoinabc.org:8333",
            "seed-abc.bitcoinforks.org:8333",
            "seed.bitprim.org:8333",
            "seed.deadalnix.me:8333",
            "seeder.criptolayer.net:8333",
        ],
    },
)

ll
drwxr-xr-x  2 username group 64B Nov  1 10:06 r
```

## Documentation

Extensive [documentation](docs/README.md)
