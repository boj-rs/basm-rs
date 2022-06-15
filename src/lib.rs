#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(once_cell)]
#![cfg_attr(not(test), no_builtins, no_std)]
extern crate alloc;

pub mod allocator;
pub mod io;
pub mod libc_string;
pub mod sorts;
pub mod syscall;
