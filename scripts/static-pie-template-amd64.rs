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
        ".quad 555d5441f0e48348h,85c0315257565341h,50c0ff50c0940fedh,1d8d4870ec834854h,\
        1674ed85000000b8h,194b8d4822c38348h,538d485950d3ff41h,0b95f50d4ff4146h,\
        74ed85d3ff000010h,8a5b50f64b8d4819h,0ff48c1ff48108811h,8948f175c3fa80c0h,\
        58d48575f50027bh,0b60fc931000000c4h,4830b60fc0ff4810h,0c1fff8144c88c0ffh,\
        0f983f476f239c2ffh,20e8ee894ce37255h,5f56f6894c000000h,89485800000016e8h,\
        28244c8d4868245ch,3145f8894df2894ch,0f7c031ed31d0ffc9h,745dfa8316b60fe1h,\
        48d0011414b60f1ah,7c05fd83c5ffc6ffh,0c783480789c80fe6h,3158096ac3d8eb04h,\
        226a5a076ace89ffh,31455841ff6a5a41h,0cdefb848c3050fc9h,83480123456789abh,\
        0b841c9315a5128ech,5941406a00003000h,6bc328c48348d0ffh,65006e0072006500h,\
        320033006c00h,0\n.asciz \"09AZaz!!#&(+--;@^`{{~VirtualAlloc\"",
        in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r11") win::GetModuleHandleW, in("r12") win::GetProcAddress,
        in("r13") b$$$$stub_base85$$$$.as_ptr(),
        in("r14") PAYLOAD.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)] fn main() { unsafe { _start() } }
// LOADER END