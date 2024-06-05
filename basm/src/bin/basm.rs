#![cfg_attr(not(test), no_builtins)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate basm_std as basm;
mod lang_items;

#[cfg_attr(test, allow(dead_code))]
#[path = "../solution.rs"]
mod solution;
