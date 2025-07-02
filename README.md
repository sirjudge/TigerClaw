# TigerClaw
Grug Brain Architecture inspired CLI tool to create easy automated API call validation tests

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

```shell
# Uses predefined advertiser and external_id - which is merchant_id in this
# case - to make a call to tigerClaw after a successful build
make run

# Uses AWS Session Manager to load the sas-data-import service to test with locally
make-run-aws

curl http://ui.d-lhr1-docker-026.dev.awin.com/idpbackend/token -d 'grant_type=client_credentials&client_id=MigrationTestClient&client_secret=211b0f6e81f9a676cc44de6308076ffb37f13a5550c4a5a8206d6f8493581972'
