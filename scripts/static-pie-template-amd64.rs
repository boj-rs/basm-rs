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
}
#[cfg(target_os = "windows")]
unsafe extern "win64" fn svc_alloc_rwx(size: usize) -> usize {
    win_api::VirtualAlloc(0, size, 0x3000 /* MEM_COMMIT | MEM_RESERVE */, 0x40 /* PAGE_EXECUTE_READWRITE */)
}
#[cfg(not(target_os = "windows"))]
unsafe extern "win64" fn svc_alloc_rwx(size: usize) -> usize {
    let ret;
    core::arch::asm!("syscall", in("rax") 9, in("rdi") 0, in("rsi") size,
        in("rdx") 0x7 /* protect */, in("r10") 0x22 /* flags */,
        in("r8") -1 /* fd */, in("r9") 0 /* offset */,
        lateout("rax") ret, out("rcx") _, out("r11") _);
    ret
}

static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
pub unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 0x52415341f0e48348,0x49c0315141525756,0x4150c0950f01f883,0xc0ec814854415450,\
        0x74058d48000000,0x814b60fc9310000,0x55f983c1ff140c88,0x4100001000b9f272,\
        0xee894cc48949d4ff,0x21e8e7894c,0x16e8f78948f6894c,0x4860c48348000000,\
        0x4df2894c20244c8d,0xff415941006af889,0xc031ed3159556ad4,0x5dfa8316b60fe1f7,\
        0x1081454b60f1b74,0xfd83c5ffc6ff48d0,0x480789c80fe57c05,0x3130c3d7eb04c783",
        ".ascii \"23456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#\\x24%&()*+-;<=>?@^_`{{|}}~\"",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r10") win_api::GetModuleHandleW,
        in("r11") win_api::GetProcAddress,
        in("r12") svc_alloc_rwx,
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