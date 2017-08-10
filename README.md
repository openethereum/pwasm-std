# wasm-std
WASM contracts standard library for Rust

It is the limited subset of Rust standard library, along with custom allocator which delegates the allocation to the runtime-defined externs.

## use

Just add a git dependency
```toml
[dendencies] 
wasm-std = { git = "https://github.com/NikVolf/wasm-std" }
```

The crate is supposed to be used on nightly Rust only, until custom allocator api stablizes in Rust.

## no_std

wasm-std is itself compiled with no_std and expected to be used withing no_std-crates/binaries, since it defines `lang_item` on it's own, which will conflict with standart library.
