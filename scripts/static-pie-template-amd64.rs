// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![crate_type="cdylib"]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(not(target_os = "windows"), no_std)]#[no_link]extern crate std as _;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))]
compile_error!("The target architecture is not supported.");
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("The target operating system is not supported.");

#[cfg(target_os = "windows")]
mod win {
    #[link(name = "kernel32")] extern "C" {
        pub fn GetModuleHandleW(lpModuleName: *const u16) -> usize;
        pub fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
    }
}
#[cfg(not(target_os = "windows"))]
mod win {
    pub const GetModuleHandleW: usize = 0;
    pub const GetProcAddress: usize = 0;
}
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 555d5441f0e48348h,0c031515257565341h,0c0ff50c0940fed85h,8d4878ec83485450h,\
        74ed85000000be1dh,4b8d4822c3834816h,8d485950d3ff4119h,0b95f50d4ff413f53h,\
        0ed85d3ff00001000h,5b50f64b8d481974h,48c1ff481088118ah,48f175c3fa80c0ffh,\
        0c489495f50027b89h,31000000c1058d48h,0fc0ff4810b60fc9h,144c88c0ff4830b6h,\
        76f239c2ffc1fff8h,894ce37255f983f4h,894c00000024e8eeh,1ae85f56f6h,\
        245c894810c48348h,894c20244c8d4860h,41c93145f8894df2h,0e1f7c031ed31d4ffh,\
        1a745dfa8316b60fh,0ff48d0011414b60fh,0e67c05fd83c5ffc6h,4c783480789c80fh,\
        0ff3158096ac3d8ebh,41226a5a076ace89h,0c931455841ff6a5ah,0abcdefb848c3050fh,\
        0ec83480123456789h,0b841c9315a5128h,0ff5941406a000030h,6bc328c48348d0h,\
        65006e00720065h,320033006ch",
        ".asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r11") win::GetModuleHandleW,
        in("r12") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END