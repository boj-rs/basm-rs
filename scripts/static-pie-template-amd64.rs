// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![crate_type = "cdylib"]
#![allow(dead_code, non_upper_case_globals)]
#![cfg_attr(not(windows), no_std)]#[no_link]extern crate std as _;

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
static mut PAYLOAD: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
unsafe fn _start() {
    let _p = (0, 0); #[cfg(windows)] let _p = (LoadLibraryA(b"kernel32\0".as_ptr()), GetProcAddress);
    core::arch::asm!(
        ".quad 50505157f0e48348h,940fff8548c03150h,5450c0ff50c589c0h,891d8d4880c48348h,\
        480c75ed85000000h,0ff2e538d4822c383h,0ff10b5c9315f50d7h,480f75ed859748d3h,\
        59226a5b57f6738dh,894802438948a4f3h,8d358d485778245ch,31a65e8d48000000h,\
        4c88acd0b60facc9h,0c238c2ffc1fff814h,4cea7255f983f476h,56f6894cd3ffee89h,\
        244c8d4858d3ff5fh,0ed31d0fff2894c38h,5d3cac92e1f7c031h,0d0010404b60f2974h,\
        0feb7c05fd83c5ffh,3158096ae2ebabc8h,226a5a076ace89ffh,31455841ff6a5a41h,\
        0cdefb848c3050fc9h,5a510123456789abh,3000b841c931h,0ffcc294c5941406ah,\
        0c340c48348d0h\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") _p.0, in("rdi") _p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") b$$$$stub_base85$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END