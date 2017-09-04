# pwasm-std

Parity WASM contracts standard library for Rust

[![Build Status](https://travis-ci.org/NikVolf/wasm-std.svg?branch=master)](https://travis-ci.org/NikVolf/wasm-std)

It is the limited subset of Rust standard library, along with custom allocator which delegates the allocation to the runtime-defined externs.

## use

Just add a git dependency
```toml
[dendencies] 
pwasm-std = { git = "https://github.com/NikVolf/pwasm-std" }
```

The crate is supposed to be used on nightly Rust only, until custom allocator api stablizes in Rust.

## no_std

pwasm-std is itself compiled with no_std and expected to be used withing no_std-crates/binaries, since it defines `lang_item` on it's own, which will conflict with standart library.
