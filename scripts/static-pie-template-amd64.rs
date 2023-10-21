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
        ".quad 0x41c5894cf0e48348,0x5141525756534154,0x27401fd8348c031,0xec8148545550c0ff,\
        0x30058d48000000c8,0x14b60fc931000001,0xf983c1ff140c8808,0xe21d8d48f27255,\
        0x257501fd83480000,0x48000000ad1d8d48,0xff41000000e60d8d,0xeb158d48c18948d3,\
        0x8949d4ff41000000,0x1000b9f9c28948c4,0x1fd8348d3ff0000,0xd8d48c389481d75,\
        0x1088118a00000077,0xfa80c0ff48c1ff48,0x490363894cf175c3,0xc0249c8948c489,\
        0xe7894cee894c0000,0xf6894c00000021e8,98397817160,0x244c8d4860c48348,\
        0x6af8894df2894c20,0x556ad4ff41594100,0xfe1f7c031ed3159,0xf1b745dfa8316b6,\
        0xff48d001081454b6,0xe57c05fd83c5ffc6,0x4c783480789c80f,0xcdefb848f8c3d7eb,\
        0xf480123456789ab,0x894828ec8348c242,0x3000b841c931ca,0x48d0ff5941406a00,\
        0x3158096ac328c483,0x226a5a076ace89ff,0x31455841ff6a5a41,0x65006bc3050fc9,\
        0x6c0065006e0072,0x6956000000320033,0x6c6c416c61757472,0x343332313000636f",
        ".ascii \"56789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
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