// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![crate_type="cdylib"]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(not(target_os = "windows"), no_std)]#[no_link]extern crate std as _;

// SOLUTION BEGIN
#[cfg(any())]
mod solution {
$$$$solution_src$$$$
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(target_arch = "x86_64"))]
compile_error!("The target architecture is not supported.");
#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
compile_error!("The target operating system is not supported.");

#[cfg(target_os = "windows")]
mod win {
    #[link(name = "kernel32")]
    extern "win64" {
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
        ".quad 41e5894cf0e48348h,5141525756534154h,0c0ff0275ed85c031h,0ec81485450c0ff50h,\
        0f41d8d48000000c8h,482574ed85000000h,8d48000000c11d8dh,0d3ff41000000fa0dh,\
        0ff158d48c18948h,0c48949d4ff410000h,1000b9f9c28948h,481d74ed85d3ff00h,\
        8d0d8d48c389h,0c1ff481088118a00h,0f175c3fa80c0ff48h,48c489490363894ch,\
        48000000c0249c89h,0c931000000c8058dh,0ff140c880814b60fh,894cf27255f983c1h,\
        20e8e7894ceeh,0e8f78948f6894c00h,60c4834800000015h,0f2894c20244c8d48h,\
        0ff41c93145f8894dh,0c031ed3159556ad4h,5dfa8316b60fe1f7h,1081454b60f1b74h,\
        0fd83c5ffc6ff48d0h,480789c80fe57c05h,48f8c3d7eb04c783h,23456789abcdefb8h,\
        0ec8348c2420f4801h,0b841c931ca894828h,5941406a00003000h,6ac328c48348d0ffh,\
        76ace89ff315809h,41ff6a5a41226a5ah,6bc3050fc9314558h,65006e0072006500h,\
        320033006c00h,6c61757472695600h,313000636f6c6c41h",
        ".ascii \"23456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r11") win::GetModuleHandleW,
        in("r12") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END