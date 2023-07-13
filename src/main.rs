#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![no_builtins]
#![no_std]
#![no_main]
extern crate alloc;

mod codegen;
mod solution;
