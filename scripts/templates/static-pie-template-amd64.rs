// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(all(target_arch = "x86_64", any(windows, target_os = "linux"))))]
compile_error!("Unsupported target architecture or operating system.");
macro_rules! p { () => { "stc" } }
#[cfg(windows)]
macro_rules! p { () => { "call LoadLibraryA;lea rdx,[rip+GetProcAddress];lea rdi,[rip+VirtualAlloc];clc" } }

static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;
#[no_mangle]
unsafe fn _start() {
    s::arch::asm!(p!(),
        ".quad 19510173000030c8h,4ce8d9f7c9h,459927e36758096ah,870d74ff8548c931h,\
        4100003000b841cah,0b2ce8956e7ff40b1h,41ff6a5a41226a07h,0c11fb0c35e050f58h,\
        99f572242cac0de0h,15bc06b242cac92h,10c4f608e8c1aad0h,5052535be3ebf775h,\
        20ec834851c1ff51h,0c93197485750d3ffh,90c9d0ff585fd3ffh",
        in("rcx") "KERNEL32\0".as_ptr(), in("r14") PAYLOAD.as_mut_ptr(), in("rsi") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END