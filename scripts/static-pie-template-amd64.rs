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
        ".quad 41c5894cf0e48348h,5141525756534154h,0ff027401fd83c031h,0c8ec8148545550c0h,\
        0f61d8d48000000h,48257501fd830000h,8d48000000c21d8dh,0d3ff41000000fb0dh,\
        100158d48c18948h,0c48949d4ff410000h,1000b9f9c28948h,1d7501fd83d3ff00h,\
        8d0d8d48c38948h,0ff481088118a0000h,75c3fa80c0ff48c1h,0c489490363894cf1h,\
        0c0249c8948h,31000000c8058d48h,140c880814b60fc9h,4cf27255f983c1ffh,\
        20e8e7894cee89h,0f78948f6894c0000h,0c4834800000015e8h,894c20244c8d4860h,\
        41c93145f8894df2h,31ed3159556ad4ffh,0fa8316b60fe1f7c0h,81454b60f1b745dh,\
        83c5ffc6ff48d001h,789c80fe57c05fdh,0f8c3d7eb04c78348h,456789abcdefb848h,\
        8348c2420f480123h,41c931ca894828ech,41406a00003000b8h,0c328c48348d0ff59h,\
        6ace89ff3158096ah,0ff6a5a41226a5a07h,0c3050fc931455841h,6e00720065006bh,\
        320033006c0065h,6175747269560000h,3000636f6c6c416ch",
        ".ascii \"123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r11") win::GetModuleHandleW,
        in("r12") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END