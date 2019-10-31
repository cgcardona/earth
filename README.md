# EARTH

```
EARTH 0.0.1
EARTH <https://www.earth.engineering>
EARTH client

USAGE:
    EARTH [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Do not show any synchronization information in the console.
        --regtest    Use a private network.
        --testnet    Use the test network.
    -V, --version    Prints version information

OPTIONS:
        --db-cache <SIZE>    Sets the database cache size.
        --port <PORT>        Listen for connections on PORT.

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    import    Import blocks from a Bitcoin Core database.
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
