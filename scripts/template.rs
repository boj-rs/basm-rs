#![crate_type="cdylib"]#![no_std]#[no_link]extern crate std;#[no_mangle]static _start:[u64;%(len)s]=[%(text)s];
