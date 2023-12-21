# chksum-hash-sha2

[![GitHub](https://img.shields.io/badge/github-chksum--rs%2Fhash--sha2-24292e?style=flat-square&logo=github "GitHub")](https://github.com/chksum-rs/hash-sha2)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/hash-sha2/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/hash-sha2/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum-hash-sha2?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum-hash-sha2/)
[![MSRV](https://img.shields.io/badge/MSRV-1.63.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/hash-sha2/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-hash-sha2/0.0.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-hash-sha2/0.0.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/hash-sha2?style=flat-square "LICENSE")](https://github.com/chksum-rs/hash-sha2/blob/master/LICENSE)

An implementation of SHA-2 hash algorithms for batch and stream computation.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum-hash-sha2 = "0.0.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum-hash-sha2
```

## Usage

Use the `hash` function for batch digest calculation.

```rust
use chksum_hash_sha2::sha2_256;

let digest = sha2_256::hash(b"example data");
assert_eq!(
    digest.to_hex_lowercase(),
    "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
);
```

Use the `default` function to create a hash instance for stream digest calculation.

```rust
use chksum_hash_sha2::sha2_512;

let digest = sha2_512::default()
    .update("example")
    .update(b"data")
    .update([0, 1, 2, 3])
    .digest();
assert_eq!(
    digest.to_hex_lowercase(),
    "57f35477757af6734892604de3846a97d2cc17cd37068373075e56a4843b3e9c83f9b435beae9fcf1da590e73e62fe20468f52ff13b095241fec77884086b090"
);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/chksum-hash-sha2/).

## Features

Cargo features are used to enable or disable specific algorithm functions.

* `224` enables SHA-2 224, accessible via the `sha2_224` module,
* `256` enables SHA-2 256, accessible via the `sha2_256` module,
* `384` enables SHA-2 384, accessible via the `sha2_384` module,
* `512` enables SHA-2 512, accessible via the `sha2_512` module.

By default all of them are enabled.

To customize your setup, turn off the default features and enable only those that you want in your `Cargo.toml` file:

```toml
[dependencies]
chksum-hash-sha2 = { version = "0.0.0", default-features = no, features = ["256", "512"] }
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum-hash-sha2 --no-default-features --features 256,512
```

## License

This crate is licensed under the MIT License.
