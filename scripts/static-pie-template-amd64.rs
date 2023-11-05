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
    fn GetModuleHandleW(lpModuleName: *const u16) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
}
static mut PAYLOAD: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
unsafe fn _start() {
    let _p = (0, 0); #[cfg(windows)] let _p = (GetModuleHandleW, GetProcAddress);
    core::arch::asm!(
        ".quad 52565057f0e48348h,940fff8548c93151h,5451c1ff51cd89c1h,971d8d4880c48348h,\
        481475ed85000000h,0ff184b8d4822c383h,0ff40538d485950d0h,0ff10b5c9315f50d7h,\
        480f75ed859748d3h,59226a5b57f6738dh,894802438948a4f3h,0a5358d485778245ch,\
        31945e8d48000000h,4c88acd0b60facc9h,0c238c2ffc1fff814h,4cea7255f983f476h,\
        56f6894cd3ffee89h,244c8d4858d3ff5fh,45f8894df2894c38h,0c031ed31d0ffc931h,\
        29745d3cac92e1f7h,0c5ffd0010404b60fh,0abc80feb7c05fd83h,89ff3158096ae2ebh,\
        5a41226a5a076aceh,0fc931455841ff6ah,89abcdefb848c305h,0c9315a5101234567h,\
        406a00003000b841h,48d0ffcc294c5941h,65006bc340c483h,6c0065006e0072h,\
        320033h\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") $$$$pe_image_base$$$$, in("rdx") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("rax") _p.0, in("rdi") _p.1, in("r14") PAYLOAD.as_mut_ptr(),
        in("r13") b$$$$stub_base85$$$$.as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END