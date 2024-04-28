//! This crate provides an implementation of SHA-2 hash functions based on [FIPS PUB 180-4: Secure Hash Standard](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf).
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-hash-sha2 = "0.0.1"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-hash-sha2
//! ```     
//!
//! # Batch Processing
//!
//! The digest of known-size data can be calculated with the `hash` function.
//!
//! ```rust
//! use chksum_hash_sha2::sha2_256;
//!
//! let digest = sha2_256::hash("example data");
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! ```
//!
//! # Stream Processing
//!
//! The digest of data streams can be calculated chunk-by-chunk with a consumer created by calling the `default` function.
//!
//! ```rust
//! // Import all necessary items
//! # use std::io;
//! # use std::path::PathBuf;
//! use std::fs::File;
//! use std::io::Read;
//!
//! use chksum_hash_sha2::sha2_512;
//!
//! # fn wrapper(path: PathBuf) -> io::Result<()> {
//! // Create a hash instance
//! let mut hash = sha2_512::default();
//!
//! // Open a file and create a buffer for incoming data
//! let mut file = File::open(path)?;
//! let mut buffer = vec![0; 64];
//!
//! // Iterate chunk by chunk
//! while let Ok(count) = file.read(&mut buffer) {
//!     // EOF reached, exit loop
//!     if count == 0 {
//!         break;
//!     }
//!
//!     // Update the hash with data
//!     hash.update(&buffer[..count]);
//! }
//!
//! // Calculate the digest
//! let digest = hash.digest();
//! // Cast the digest to hex and compare
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "ed59c5759a9ece516cec0c0623142d0e9fe70a27d750eee7fd38f4550d50addd873d0fa1a51fc823c1e3d5cada203f4a05d8325caacb7d3e0727a701f3f07e5f"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Algorithms
//!
//! ## SHA-2 224
//!
//! ```rust
//! use chksum_hash_sha2::sha2_224;
//!
//! let digest = sha2_224::hash("example data");
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "90382cbfda2656313ad61fd74b32ddfa4bcc118f660bd4fba9228ced"
//! );
//! ```
//!
//! ## SHA-2 256
//!
//! ```rust
//! use chksum_hash_sha2::sha2_256;
//!
//! let digest = sha2_256::hash("example data");
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "44752f37272e944fd2c913a35342eaccdd1aaf189bae50676b301ab213fc5061"
//! );
//! ```
//!
//! ## SHA-2 384
//!
//! ```rust
//! use chksum_hash_sha2::sha2_384;
//!
//! let digest = sha2_384::hash("example data");
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "12ecdfd463a85a301b7c29a43bf4b19cdfc6e5e86a5f40396aa6ae3368a7e5b0ed31f3bef2eb3071577ba610b4ed1cb8"
//! );
//! ```
//!
//! ## SHA-2 512
//!
//! ```rust
//! use chksum_hash_sha2::sha2_512;
//!
//! let digest = sha2_512::hash("example data");
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "ed59c5759a9ece516cec0c0623142d0e9fe70a27d750eee7fd38f4550d50addd873d0fa1a51fc823c1e3d5cada203f4a05d8325caacb7d3e0727a701f3f07e5f"
//! );
//! ```
//!
//! # Features
//!
//! Cargo features are utilized to enable or disable specific SHA-2 algorithm variants.
//!
//! * `224` enables SHA-2 224, accessible via the [`sha2_224`] module.
//! * `256` enables SHA-2 256, accessible via the [`sha2_256`] module.
//! * `384` enables SHA-2 384, accessible via the [`sha2_384`] module.
//! * `512` enables SHA-2 512, accessible via the [`sha2_512`] module.
//!
//! By default, all of these features are enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum-hash-sha2 = { version = "0.0.1", default-features = false, features = ["256", "512"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-hash-sha2 --no-default-features --features 256,512
//! ```
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

#[cfg(feature = "224")]
#[doc(no_inline)]
pub use chksum_hash_sha2_224 as sha2_224;
#[cfg(feature = "256")]
#[doc(no_inline)]
pub use chksum_hash_sha2_256 as sha2_256;
#[cfg(feature = "384")]
#[doc(no_inline)]
pub use chksum_hash_sha2_384 as sha2_384;
#[cfg(feature = "512")]
#[doc(no_inline)]
pub use chksum_hash_sha2_512 as sha2_512;
