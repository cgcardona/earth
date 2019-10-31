# EARTH

```
EARTH 0.0.1
EARTH <https://www.earth.engineering>
EARTH client

USAGE:
    earth [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Don't show any synchronization information in the console
        --regtest    Use a private network.
        --testnet    Use the test network.
    -V, --version    Prints version information

OPTIONS:
    -d, --data-dir <PATH>               Specify the database and configuration directory PATH.
        --db-cache <SIZE>               Sets the database cache size
        --port <PORT>                   Listen for connections on PORT.
        --verification-level <LEVEL>    Set the blocks verification level to full (default), header (scripts are not
                                        verified), or none (no verification at all)

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    import      Import blocks from a database.
    rollback    Rollback the database to block hash or number
```

## Usage

```
Configuration: Ok(
    Configuration {
        network: Testnet,
        port: 18332,
        db_cache: 512,
        user_agent: "/EARTH:0.0.1/",
        quiet: false,
        data_dir: Some(
            "foobar",
        ),
    },
)
IMPORT: ArgMatches {
    args: {
        "PATH": MatchedArg {
            occurs: 1,
            indices: [
                1,
            ],
            vals: [
                "path",
            ],
        },
    },
    subcommand: None,
    usage: Some(
        "USAGE:\n    earth import <PATH>",
    ),
}, 0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae180101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000, path
```

## Documentation

Extensive [documentation](docs/README.md)
