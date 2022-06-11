#![crate_type="cdylib"]#![no_std]#[no_link]extern crate std;#[link_section=".text"]#[no_mangle]static _start:[u64;%(len)s]=[%(text)s];
