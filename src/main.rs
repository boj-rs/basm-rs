#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(once_cell)]
#![cfg_attr(not(test), feature(alloc_error_handler), no_builtins, no_std, no_main)]
extern crate alloc;

#[cfg(not(test))]
mod allocator;
#[cfg(not(test))]
mod codegen;
#[cfg(not(test))]
mod solution;

#[allow(dead_code)]
mod io;
#[allow(dead_code)]
mod sorts;
#[allow(dead_code)]
mod syscall;
