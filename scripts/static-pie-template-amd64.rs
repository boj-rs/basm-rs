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
        pub fn VirtualAlloc(lpAddress: usize, dwSize: usize, flAllocationType: u32, flProtect: u32) -> usize;
    }
}
#[cfg(not(target_os = "windows"))]
mod win_api {
    pub const GetModuleHandleW: usize = 0;
    pub const GetProcAddress: usize = 0;
    pub const VirtualAlloc: usize = 0;
}
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
pub unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 0x41c5894cf0e48348,0x5141525756524153,0x27401fd8348c031,0xec8148545550c0ff,\
        0xfb058d48000000c8,0x14b60fc931000000,0xf983c1ff140c8808,0xcc1d8d48f27255,\
        0xa7501fd83480000,0x4c000000911d8d48,0xff00001000b9e289,0x481d7501fd8348d3,\
        0x7a0d8d48c389,0xc1ff481088118a00,0xf175c3fa80c0ff48,0x48c489490363894c,\
        0x4c000000c0249c89,0x21e8e7894cee89,0xf78948f6894c0000,0xc4834800000016e8,\
        0x894c20244c8d4860,0x5941006af8894df2,0xed3159556ad4ff41,0x8316b60fe1f7c031,\
        0x1454b60f1b745dfa,0xc5ffc6ff48d00108,0x89c80fe57c05fd83,0xc3d7eb04c7834807,\
        0xcdefb848f801ebf9,0xf480123456789ab,0x894828ec8348c242,0x3000b841c931ca,\
        0xff00000040b94100,0x96ac328c48348d0,0x5a076ace89ff3158,0x5841ff6a5a41226a,\
        0x3130c3050fc93145",
        ".ascii \"23456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r10") win_api::GetModuleHandleW,
        in("r11") win_api::GetProcAddress,
        in("r12") win_api::VirtualAlloc,
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