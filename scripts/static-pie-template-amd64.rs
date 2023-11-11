// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

#![crate_type = "cdylib"] // To compile on Windows, remove this line or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as _;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))]
compile_error!("The target architecture is not supported.");
#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("The target operating system is not supported.");

#[cfg(windows)]
extern "C" {
    fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
}
static mut PAYLOAD: [u8; $$$$binary_base91_len$$$$] = *br$$$$binary_base91$$$$;

#[no_mangle]
unsafe fn _start() {
    let p = (0, 0); #[cfg(windows)] let p = (LoadLibraryA("kernel32\0".as_ptr()), GetProcAddress);
    core::arch::asm!(
        ".quad 50505157f0e48348h,940fff8548c03150h,5450c0ff50c589c0h,6d1d8d4880c48348h,\
        85e07b8d4ch<<24,4822c383480c75edh,315f50d7ff1e538dh,859748d3ff10b5c9h,\
        57f6738d480f75edh,8948a4f359236a5bh,5778245c89480243h,0d7ff41ee894c5641h,\
        5ad7ff41575f565eh,0d0ff38244c8d4858h,2cac0de0c1581f6ah,242cac92992c7224h,\
        0e8c1aad0015bc06bh,0e3ebf77510c4f608h,6ace89ff3158096ah,0ff6a5a41226a5a07h,\
        0c3050fc931455841h,47176,0b841c9315a510000h,5941406a00003000h,\
        8348d0ff28ec8348h,0c328c4h\n.asciz\"VirtualAlloc\"",
        in("rcx") p.0, in("rdi") p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") r$$$$stub_base91$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END