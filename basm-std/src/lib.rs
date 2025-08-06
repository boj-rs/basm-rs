#![feature(clone_to_uninit)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(test)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(rustfmt, rustfmt_skip)] // temporary fix to keep compiler_builtins at the top to avoid linker errors

// "nintendo" indicates the x86_64-unknown-linux-gnu-short target
// (picked random word to avoid rustc warning)
#[cfg(not(target_vendor = "nintendo"))]
extern crate compiler_builtins;
extern crate alloc;
#[cfg(test)]
extern crate test;

pub mod collections;
pub mod graph;
pub mod math;
pub mod platform;
pub mod serialization;
pub mod sorts;
pub mod strings;
pub mod utils;
