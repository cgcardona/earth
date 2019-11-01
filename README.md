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

**WARNING** `earth --data-dir foobar` will create a `./foobar/` directory in the root directory from which you ran the command. It will create recursive directories: `earth --data-dir foo/bar`

If you don't pass in a `--data-dir` flag then a `./data-dir/` directory will be created in the root directory from which you ran the command.

```
./target/debug/earth
configuration: Ok(
    Configuration {
        network: Mainnet,
        data_dir: None,
        port: 8332,
        db_cache: 512,
        user_agent: "/EARTH:0.0.1/",
        quiet: false,
    },
)

ll
drwxr-xr-x  2 username group 64B Nov  1 10:06 data-dir
```

## Documentation

Extensive [documentation](docs/README.md)
