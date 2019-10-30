# Contributing

## Install

For Unix systems run

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rust`, `rustup` and `cargo`. Cargo is rust's package manager.

## Installing packages

[Crates.io](https://crates.io) is a community registry for rust packages, also known as crates. Crates and cargo work much like npm in the node.js ecosystem.

## Rustup

[Rustup](https://github.com/rust-lang/rustup.rs) is the Rust installer and version management tool.

### Update

When a new version of Rust is released, you can type rustup update to update to it:

```
rustup update
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: checking for self-updates

  stable-x86_64-apple-darwin unchanged - rustc 1.38.0 (625451e37 2019-09-23)
```

### Nightly

`rustup` gives you easy access to the nightly compiler and its [experimental features](https://doc.rust-lang.org/unstable-book/). To add it just run rustup toolchain install nightly:

```
rustup toolchain install nightly
info: syncing channel updates for 'nightly-x86_64-apple-darwin'
info: latest update on 2019-10-30, rust version 1.40.0-nightly (aa69777ea 2019-10-29)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 11.9 MiB /  11.9 MiB (100 %)   5.1 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 16.2 MiB /  16.2 MiB (100 %)   5.0 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 54.8 MiB /  54.8 MiB (100 %)   5.1 MiB/s in 14s ETA:  0s
info: downloading component 'rustc-dev'
160.8 MiB / 160.8 MiB (100 %)   4.2 MiB/s in 37s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 11.9 MiB /  11.9 MiB (100 %)   4.5 MiB/s in  2s ETA:  0s
info: installing component 'rust-std'
info: installing component 'rustc'
 54.8 MiB /  54.8 MiB (100 %)  13.0 MiB/s in  4s ETA:  0s
info: installing component 'rustc-dev'
160.8 MiB / 160.8 MiB (100 %)  30.5 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'

  nightly-x86_64-apple-darwin installed - rustc 1.40.0-nightly (aa69777ea 2019-10-29)

info: checking for self-updates
```

Now Rust nightly is installed, but not activated. To test it out you can run a command from the nightly toolchain like

```
rustup run nightly rustc --version
rustc 1.40.0-nightly (aa69777ea 2019-10-29)
```

### Default

To make the `nightly` build your default:

```
rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin unchanged - rustc 1.40.0-nightly (aa69777ea 2019-10-29)

```

To make the `stable` build your default:

```
rustup default stable
info: using existing install for 'stable-x86_64-apple-darwin'
info: default toolchain set to 'stable-x86_64-apple-darwin'

  stable-x86_64-apple-darwin unchanged - rustc 1.38.0 (625451e37 2019-09-23)
```
