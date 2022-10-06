#![crate_type="cdylib"]#![no_std]#[no_link]extern crate std;#[repr(align(32))]struct C([u64;%(len)s]);#[link_section=".text"]#[no_mangle]static _start:C=C([%(text)s]);
