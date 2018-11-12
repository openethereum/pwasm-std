# owasm-std

Oasis WASM contracts standard library for Rust

[Documentation](https://docs.rs/crate/owasm-std/)

`owasm-std` is a limited subset of the Rust standard library, along with a custom allocator which delegates the allocation to the runtime-defined externs.

This crate is forked from [paritytech/pwasm-std](https://github.com/paritytech/pwasm-std).

We are big fans of the work [Parity](https://www.parity.io/) is doing to advance the state of Ethereum and smart contract development. In order to further experiment with new features including new opcodes and gas models, we have forked these repositories.

In the spirit of open-source, we intend to contribute bug fixes and new features upstream and will make our changes available to the community.


## Use

Just add a dependency
```toml
[dependencies]
owasm-std = "0.12"
```

Test `owasm-std` with

```
cargo test --features=test
```

The crate is supposed to be used on nightly Rust only, until the custom allocator api stablizes in Rust.

## no_std

`owasm-std` is itself compiled with no_std and expected to be used within no_std-crates/binaries, since it defines `lang_item`-s on it's own, which will conflict with standard library.

But for test scenarios it can be compiled with feature "std" and auxiliary crate "owasm-test" to support testing of contracts' internal logic.

# License

`owasm-std` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in owasm-std by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
