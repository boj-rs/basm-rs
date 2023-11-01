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
        ".quad 555d5441f0e48348h,0c031515257565341h,0ff50c0ff0275ed85h,0c8ec81485450c0h,\
        0c21d8d480000h,0c383481974ed8500h,0d3ff41274b8d4819h,0ff4150538d485950h,\
        0b9f95a505f50d4h,74ed85d3ff000010h,8a5b50ff4b8d4819h,0ff48c1ff48108811h,\
        8948f175c3fa80c0h,58d48c48949037bh,0b60fc931000000cch,4830b60fc0ff4810h,\
        0ffc1ff140c88c0ffh,55f983f576f239c2h,0e7894cee894ce472h,0f6894c00000024e8h,\
        480000001ae85f56h,60245c894860c483h,0f2894c20244c8d48h,0ff41c93145f8894dh,\
        0fe1f7c031ed31d4h,0f1b745dfa8316b6h,0ff48d001081454b6h,0e57c05fd83c5ffc6h,\
        4c783480789c80fh,0ff3158096ac3d7ebh,41226a5a076ace89h,0c931455841ff6a5ah,\
        0cdefb848f8c3050fh,0f480123456789abh,5a5128ec8348c242h,3000b841c931h,\
        8348d0ff5941406ah,720065006bc328c4h,33006c0065006e00h,3200h",
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