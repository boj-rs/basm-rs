#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(once_cell)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod allocator;
pub mod collections;
pub mod graph;
pub mod io;
pub mod libc_string;
pub mod sorts;
pub mod syscall;
