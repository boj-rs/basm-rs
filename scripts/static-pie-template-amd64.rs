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
static mut PAYLOAD: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 52565057f0e48348h,940fff8548c93151h,5451c1ff51cd89c1h,0b31d8d4880c48348h,\
        481475ed85000000h,0ff194b8d4822c383h,0ff44538d485950d0h,0ff10b5c9315f50d7h,\
        4b8d481975ed85d3h,481088118a5b50f6h,0c3fa80c0ff48c1ffh,8948027b8948f175h,\
        8d48575f5078245ch,588d48000000bb05h,0ff4810b60fc93185h,88c0ff4830b60fc0h,\
        39c2ffc1fff8144ch,0e37255f983f476f2h,0f6894cd3ffee894ch,4c8d4858d3ff5f56h,\
        0f8894df2894c3824h,31ed31d0ffc93145h,0fa8316b60fe1f7c0h,11414b60f1a745dh,\
        0fd83c5ffc6ff48d0h,480789c80fe67c05h,96ac3d8eb04c783h,5a076ace89ff3158h,\
        5841ff6a5a41226ah,0b848c3050fc93145h,123456789abcdefh,0c9315a5128ec8348h,\
        406a00003000b841h,28c48348d0ff5941h,6e00720065006bc3h,320033006c006500h,\
        0\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rcx") $$$$pe_image_base$$$$, in("rdx") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("rax") win::GetModuleHandleW, in("rdi") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") PAYLOAD.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END