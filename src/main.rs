#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(once_cell)]
#![feature(alloc_error_handler)]
#![no_builtins]
#![no_std]
#![no_main]
extern crate alloc;

mod codegen;
mod solution;
