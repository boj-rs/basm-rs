// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this line or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;use s::arch::*;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))] compile_error!("Unsupported target architecture.");
#[cfg(not(any(windows, target_os = "linux")))] compile_error!("Unsupported target operating system.");

#[cfg(windows)]
global_asm!("p:enter 32,0;lea rcx,[rip+k];call LoadLibraryA;lea rcx,[rip+GetProcAddress];leave;ret;k:.asciz\"kernel32\"");
#[cfg(not(windows))]
global_asm!("p:mov rcx,0;ret");

static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;
#[no_mangle]
unsafe fn _start() {
    asm!(
        "call p;.quad 0e48348000050c853h,48d23151509148f0h,0c2ff52c2940fc085h,358d4820ec834852h,\
        74c0854800000045h,50d0ff4f568d4806h,8948d6ff59016a5bh,48b8669748582444h,\
        6aab489348ab66b8h,41575b56a4f3592eh,565ed3ffee894c56h,8d48585ad3ff575fh,\
        0eb5bc9d0ff20244ch,5a511074c085485ch,3000b841c931h,0b056e0ff5941406ah,\
        5a076ace89ff3109h,5841ff6a5a41226ah,6ac35e050fc93145h,242cac0de0c1581fh,\
        6b242cac9299f472h,8e8c1aad0015bc0h,0e3ebf77510c4f6h;.asciz\"VirtualAlloc\"",
        in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END