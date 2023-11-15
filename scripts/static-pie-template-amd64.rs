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
        ".quad 0e48348000050c853h,0c08548d2315150f0h,4852c2ff52c2940fh,6e1d8d4820ec83h,\
        8548e07b8d4c0000h,0ff30538d480674c0h,0ff10b5c9315f50d0h,505b575e539748d3h,\
        4858ab66b848b866h,8948a4f3592f6aabh,894c56415758245ch,575f565ed7ff41eeh,\
        4c8d48585ad7ff41h,5deb5bc9d0ff2024h,2cac0de0c1581f6ah,242cac9299437224h,\
        0e8c1aad0015bc06bh,0e3ebf77510c4f608h,315a511074c08548h,6a00003000b841c9h,\
        96a57e0ff594140h,5a076ace89ff3158h,5841ff6a5a41226ah,0c35f050fc93145h",
        ".asciz\"VirtualAlloc\"", in("rcx") p.0, in("rax") p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END