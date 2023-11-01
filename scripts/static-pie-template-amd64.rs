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
        74ed85000000bc1dh,4b8d4822c3834816h,8d485950d3ff4119h,0b95f50d4ff414153h,\
        0ed85d3ff00001000h,5b50f64b8d481974h,48c1ff481088118ah,48f175c3fa80c0ffh,\
        8d48575f50027b89h,0fc931000000c305h,30b60fc0ff4810b6h,0fff8144c88c0ff48h,\
        83f476f239c2ffc1h,0e8ee894ce37255f9h,56f6894c00000024h,48580000001ae85fh,\
        60245c894810c483h,0f2894c20244c8d48h,0d0ffc93145f8894dh,0b60fe1f7c031ed31h,\
        0b60f1a745dfa8316h,0ffc6ff48d0011414h,0c80fe67c05fd83c5h,0d8eb04c783480789h,\
        0ce89ff3158096ac3h,6a5a41226a5a076ah,50fc931455841ffh,6789abcdefb848c3h,\
        5128ec8348012345h,3000b841c9315ah,48d0ff5941406a00h,65006bc328c483h,\
        6c0065006e0072h,320033h\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r11") win::GetModuleHandleW, in("r12") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END