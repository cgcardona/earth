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
./target/debug/EARTH --quiet  --testnet
Ok(
    Configuration {
        network: Testnet,
        port: 18332,
        db_cache: 512,
        user_agent: "/EARTH:0.0.1/",
        quiet: true,
    },
)
```

## Documentation

Extensive [documentation](docs/README.md)
