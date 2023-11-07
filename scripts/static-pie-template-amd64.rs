// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![crate_type = "cdylib"]
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
        ".quad 50505157f0e48348h,940fff8548c03150h,5450c0ff50c589c0h,881d8d4880c48348h,\
        480c75ed85h<<24,0ff2f538d4822c383h,0ff10b5c9315f50d7h,480f75ed859748d3h,\
        59236a5b57f6738dh,894802438948a4f3h,8d4856415778245ch,5e8d480000008b35h,\
        0acd0b60facc931a4h,38c2ffc1ff140c88h,0eb7255f983f576c2h,5f565ed3ffee894ch,\
        4c8d48585ad3ff57h,0c031ed31d0ff3824h,2a745d3cac92e1f7h,0ffd001080444b60fh,\
        0c80fea7c05fd83c5h,0ff3158096ae1ebabh,41226a5a076ace89h,0c931455841ff6a5ah,\
        0b848c3050fh,315a51h<<40,6a00003000b841c9h,0ff28ec8348594140h,\
        0c328c48348d0h\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") _p.0, in("rdi") _p.1, in("r14") PAYLOAD.as_mut_ptr(), in("r13") b$$$$stub_base85$$$$.as_ptr()
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END