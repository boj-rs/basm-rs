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
macro_rules! p { () => { "lea rcx,[rip+120];call LoadLibraryA;lea rdx,[rip+GetProcAddress];lea rdi,[rip+VirtualAlloc];clc" } }
#[cfg(not(windows))]
macro_rules! p { () => { "stc" } }

static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;
#[no_mangle]
unsafe fn _start() {
    s::arch::asm!(p!(),
        ".quad 19510172000050c8h,0c1ff515052d9f7c9h,53e820ec834851h,31459958096a0000h,\
        0ca870d74ff8548c9h,0b14100003000b841h,7b2ce8956e7ff40h,5841ff6a5a41226ah,\
        0c1581f6ac35e050fh,99f472242cac0de0h,15bc06b242cac92h,10c4f608e8c1aad0h,\
        6e72656be3ebf775h,0d6ff5e0032336c65h,9748602444894857h,4858ab66bf48b866h,\
        4957a4f3592a6aabh,5e5641d5ff41f587h,585ad5ff41575f56h,0c9d0ff20244c8d48h",
        in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END