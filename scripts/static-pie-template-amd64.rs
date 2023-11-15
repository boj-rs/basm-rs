// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, remove this line or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as _;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))] compile_error!("Unsupported target architecture.");
#[cfg(not(any(windows, target_os = "linux")))] compile_error!("Unsupported target operating system.");
#[cfg(windows)] extern "C" {
    fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
}
static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;

#[no_mangle]
unsafe fn _start() {
    let p = (0, 0); #[cfg(windows)] let p = (LoadLibraryA("kernel32\0".as_ptr()), GetProcAddress);
    core::arch::asm!(
        ".quad 0e48348000050c853h,0c08548d2315150f0h,4852c2ff52c2940fh,46358d4820ec83h,\
        480674c085480000h,315b50d0ff50568dh,448948d6ff10b5c9h,0b848b86697485824h,\
        2f6aab489348ab66h,5641575b56a4f359h,5f565ed3ffee894ch,4c8d48585ad3ff57h,\
        5deb5bc9d0ff2024h,315a511074c08548h,6a00003000b841c9h,96a56e0ff594140h,\
        5a076ace89ff3158h,5841ff6a5a41226ah,6ac35e050fc93145h,242cac0de0c1581fh,\
        6b242cac9299f472h,8e8c1aad0015bc0h,0e3ebf77510c4f6h\n.asciz\"VirtualAlloc\"",
        in("rcx") p.0, in("rax") p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END