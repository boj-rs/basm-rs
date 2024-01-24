#![feature(rustc_private)]
#![feature(fn_align)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(naked_functions)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;
#[cfg(not(test))]
extern crate compiler_builtins;

pub mod collections;
pub mod graph;
pub mod math;
pub mod platform;
pub mod serialization;
pub mod sorts;
pub mod strings;
