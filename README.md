# pwasm-std

Parity WASM contracts standard library for Rust

[![Build Status](https://travis-ci.org/paritytech/pwasm-std.svg?branch=master)](https://travis-ci.org/NikVolf/pwasm-std)

It is a limited subset of the Rust standard library, along with a custom allocator which delegates the allocation to the runtime-defined externs.

## use

Just add a git dependency
```toml
[dendencies] 
pwasm-std = { git = "https://github.com/paritytech/pwasm-std" }
```

The crate is supposed to be used on nightly Rust only, until the custom allocator api stablizes in Rust.

## no_std

`pwasm-std` is itself compiled with no_std and expected to be used within no_std-crates/binaries, since it defines `lang_item`-s on it's own, which will conflict with standard library.

But for test scenarios it can be compiled with feature "std" and auxiliary crate "pwasm-test" to support testing of contracts' internal logic.
