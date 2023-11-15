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
        ".quad 0e48348000050c853h,0c08548d2315150f0h,4852c2ff52c2940fh,45358d4820ec83h,\
        480674c085480000h,6a5b50d0ff51568dh,24448948d6ff5901h,66b848b866974858h,\
        592e6aab489348abh,4c5641575b56a4f3h,575f565ed3ffee89h,244c8d48585ad3ffh,\
        485eeb5bc9d0ff20h,0c9315a511074c085h,406a00003000b841h,3109b056e0ff5941h,\
        226a5a076ace89ffh,31455841ff6a5a41h,581f6ac35e050fc9h,0f472242cac0de0c1h,\
        5bc06b242cac9299h,0c4f608e8c1aad001h,0e3ebf77510h;.asciz\"VirtualAlloc\"",
        in("rcx") p.0, in("rax") p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END