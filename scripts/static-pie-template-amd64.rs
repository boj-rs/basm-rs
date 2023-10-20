// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
#![allow(non_snake_case, non_upper_case_globals)]
#![crate_type="cdylib"]
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

static STUB_BASE85: [u8; $$$$stub_base85_len$$$$] = *b$$$$stub_base85$$$$;
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
pub unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 0xb0ec8148f0e48348,0x70248c8d48000001,0xc03101894c000001,0x48c0950f01f88349",
        ".quad 0x481049894c084189,0x4820798948185189,0x4c3051894c287189,0x2024848d48385989",
        ".quad 0x484060894c000001,0xac058d48484889,0x3120244c8d480000,0x111c881814b60fdb",
        ".quad 0xb9f27255fb83c3ff,0x49d4ff4100001000,0x4c20244c8d48c489,0x29e8e7894cee89",
        ".quad 0xf78948f6894c0000,0x8c8d480000001ee8,0xf2894c0000012024,0xb941f8894d",
        ".quad 0x100c4814800,0x55bbd4ff41,0x45745df88306b60f,0xf71104b60f16b60f",
        ".quad 0x14b60f0156b60fe3,0x56b60fe3f7d00111,0xf7d0011114b60f02,0x14b60f0356b60fe3",
        ".quad 0x56b60fe3f7d00111,0xfd0011114b60f04,0x4805c683480789c8,0x3130c3b3eb04c783",
        ".quad 0x3938373635343332,0x4847464544434241,0x504f4e4d4c4b4a49,0x5857565554535251",
        ".quad 0x6665646362615a59,0x6e6d6c6b6a696867,0x767574737271706f,0x252423217a797877",
        ".quad 0x3c3b2d2b2a292826,0x7b605f5e403f3e3d,8289660",
        in("r9") $$$$leading_unused_bytes$$$$, in("rdx") $$$$pe_image_base$$$$, in("rdi") $$$$pe_off_reloc$$$$, in("rsi") $$$$pe_size_reloc$$$$, in("r15") $$$$entrypoint_offset$$$$,
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r10") win_api::GetModuleHandleW,
        in("r11") win_api::GetProcAddress,
        in("r12") svc_alloc_rwx,
        in("r13") STUB_BASE85.as_ptr(),
        in("r14") BINARY_BASE85.as_mut_ptr(),
        options(noreturn)
    )
}
#[allow(dead_code)]
fn main() { unsafe { _start() } }
//==============================================================================
// LOADER END
//==============================================================================