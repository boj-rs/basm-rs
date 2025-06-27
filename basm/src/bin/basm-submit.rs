#![feature(fn_align)]
#![cfg_attr(not(test), no_builtins)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(rustfmt, rustfmt_skip)] // temporary fix to keep compiler_builtins at the top to avoid linker errors

// "nintendo" indicates the x86_64-unknown-linux-gnu-short target
// (picked random word to avoid rustc warning)
#[cfg(not(target_vendor = "nintendo"))]
extern crate compiler_builtins;
extern crate alloc;
extern crate basm_std as basm;
mod lang_items;

#[cfg_attr(test, allow(dead_code))]
#[path = "../solution.rs"]
mod solution;
