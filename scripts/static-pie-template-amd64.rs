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
        ".quad 555d5441f0e48348h,0c031515257565341h,0ff50c0ff0275ed85h,0c8ec81485450c0h,\
        0b71d8d480000h,0c383481974ed8500h,0d3ff41274b8d4819h,0ff4139538d485950h,\
        0b9f95a505f50d4h,74ed85d3ff000010h,8a5b50ff4b8d4819h,0ff48c1ff48108811h,\
        8948f175c3fa80c0h,58d48c48949037bh,0b60fc931000000cbh,83c1ff140c880814h,\
        4cee894cf27255f9h,4c00000024e8e789h,1ae85f56f689h,5c894860c4834800h,\
        4c20244c8d486024h,0c93145f8894df289h,0ed3159556ad4ff41h,8316b60fe1f7c031h,\
        1454b60f1b745dfah,0c5ffc6ff48d00108h,89c80fe57c05fd83h,0c3d7eb04c7834807h,\
        6ace89ff3158096ah,0ff6a5a41226a5a07h,0c3050fc931455841h,6789abcdefb848f8h,\
        48c2420f48012345h,41c9315a5128ec83h,41406a00003000b8h,0c328c48348d0ff59h,\
        6e00720065006bh,320033006c0065h,6175747269560000h,3000636f6c6c416ch",
        ".ascii \"123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
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