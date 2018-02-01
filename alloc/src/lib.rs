#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]
#![crate_type = "rlib"]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(allocator_api)]

//! Custom allocator crate for wasm

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
