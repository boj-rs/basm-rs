// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![crate_type="cdylib"]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(not(target_os = "windows"), no_std)]#[no_link]extern crate std as std2;

//==============================================================================
// SOLUTION BEGIN
//==============================================================================
#[cfg(any())]
mod solution {
$$$$solution_src$$$$
}
//==============================================================================
// SOLUTION END
//==============================================================================

//==============================================================================
// LOADER BEGIN
//==============================================================================
#[cfg(not(target_arch = "x86_64"))]
compile_error!("The target architecture is not supported.");
#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
compile_error!("The target operating system is not supported.");

#[cfg(target_os = "windows")]
mod win_api {
    #[link(name = "kernel32")]
    extern "win64" {
        pub fn GetModuleHandleW(lpModuleName: *const u16) -> usize;
        pub fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
    }
}
#[cfg(not(target_os = "windows"))]
mod win_api {
    pub const GetModuleHandleW: usize = 0;
    pub const GetProcAddress: usize = 0;
}
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
pub unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 0x41c5894cf0e48348,0x5141525756534154,0xff027401fd83c031,0xc8ec8148545550c0,\
        0xf61d8d48000000,0x48257501fd830000,0x8d48000000c21d8d,0xd3ff41000000fb0d,\
        0x100158d48c18948,0xc48949d4ff410000,0x1000b9f9c28948,0x1d7501fd83d3ff00,\
        0x8d0d8d48c38948,0xff481088118a0000,0x75c3fa80c0ff48c1,0xc489490363894cf1,\
        0xc0249c8948,0x31000000c8058d48,0x140c880814b60fc9,0x4cf27255f983c1ff,\
        0x20e8e7894cee89,0xf78948f6894c0000,0xc4834800000015e8,0x894c20244c8d4860,\
        0x41c93145f8894df2,0x31ed3159556ad4ff,0xfa8316b60fe1f7c0,0x81454b60f1b745d,\
        0x83c5ffc6ff48d001,0x789c80fe57c05fd,0xf8c3d7eb04c78348,0x456789abcdefb848,\
        0x8348c2420f480123,0x41c931ca894828ec,0x41406a00003000b8,0xc328c48348d0ff59,\
        0x6ace89ff3158096a,0xff6a5a41226a5a07,0xc3050fc931455841,0x6e00720065006b,\
        0x320033006c0065,0x6175747269560000,0x3000636f6c6c416c",
        ".ascii \"123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r11") win_api::GetModuleHandleW,
        in("r12") win_api::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)]
fn main() { unsafe { _start() } }
//==============================================================================
// LOADER END
//==============================================================================