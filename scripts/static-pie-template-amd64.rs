// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this line or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))] compile_error!("Unsupported target architecture.");
#[cfg(not(any(windows, target_os = "linux")))] compile_error!("Unsupported target operating system.");

#[cfg(windows)]
macro_rules! p { () => { "lea rcx,[rip+122];call LoadLibraryA;lea rcx,[rip+GetProcAddress];lea rdi,[rip+VirtualAlloc];clc" } }
#[cfg(not(windows))]
macro_rules! p { () => { "stc" } }

static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;
#[no_mangle]
unsafe fn _start() {
    s::arch::asm!(p!(),
        ".quad 0f7db19000050c853h,535051f0e48348dbh,0e820ec834853c3ffh,9958096a00000053h,\
        0d74ff8548c93145h,3000b841ca87h,0ce8956e7ff40b141h,0ff6a5a41226a07b2h,\
        1f6ac35e050f5841h,72242cac0de0c158h,0c06b242cac9299f4h,0f608e8c1aad0015bh,\
        656be3ebf77510c4h,5e0032336c656e72h,48d6ff59016a5b57h,0b866974858244489h,\
        0ab489348ab66bf48h,575b56a4f3592a6ah,5e5641d3ffee894ch,48585ad3ff575f56h,\
        5bc9d0ff20244c8dh", in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END