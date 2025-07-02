# TigerClaw
Grug Brain Architecture inspired CLI tool to create a fully automated API testing suite

# Build
## Install Rust Nightly
[Rust Up](https://www.rust-lang.org/tools/install)
```shell
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# using rustup Install individual parts
rustup toolchain install nightly
rustup install nightly
rustup default nightly
rustup override set nightly
```

# Run + Test
Makefile provides two commands, one for running the tool and a second for running the aws session manager command
