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
        ".quad 48e4894954415355h,5750ec8348f0e483h,940fff8548c03151h,4850c0ff50c589c0h,\
        7a1d8d4820ec83h,0ed85e07b8d4c0000h,8d4818c383480c75h,0c9315f50d7ff1453h,\
        0ed859748d3ff10b5h,66505b575e531375h,0ab4858ab66b848b8h,5c8948a4f359106ah,\
        0ee894c5641575824h,41575f565ed7ff41h,244c8d48585ad7ffh,5c41e4894cd0ff20h,\
        0c1581f6a59eb5d5bh,992c72242cac0de0h,15bc06b242cac92h,10c4f608e8c1aad0h,\
        3158096ae3ebf775h,226a5a076ace89ffh,31455841ff6a5a41h,0c9315a51c3050fc9h,\
        406a00003000b841h,0e0ff5941h\n.asciz\"VirtualAlloc\"",
        in("rcx") p.0, in("rdi") p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start(); } }
// LOADER END