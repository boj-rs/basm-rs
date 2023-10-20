// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box! See: https://doc.rust-lang.org/book/

// IMPORTANT: To compile on Windows, change 'cdylib' on the next line to 'bin' or pass '--crate-type=bin' to rustc to avoid creating a DLL.
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
#[allow(non_snake_case)]
mod win_api {
    #[link(name = "kernel32")]
    extern "win64" {
        pub fn GetModuleHandleW(lpModuleName: *const u16) -> usize;
        pub fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
        pub fn VirtualAlloc(lpAddress: usize, dwSize: usize, flAllocationType: u32, flProtect: u32) -> usize;
    }
}
#[cfg(not(target_os = "windows"))]
#[allow(non_snake_case, non_upper_case_globals)]
mod win_api {
    pub const GetModuleHandleW: usize = 0;
    pub const GetProcAddress: usize = 0;
    pub fn VirtualAlloc(_lpAddress: usize, _dwSize: usize, _flAllocationType: u32, _flProtect: u32) -> usize { 0 }
}

unsafe extern "win64" fn svc_alloc_rwx(size: usize) -> *mut u8 {
    let ret;
    if cfg!(windows) {
        ret = win_api::VirtualAlloc(0, size,
            0x00003000 /* MEM_COMMIT | MEM_RESERVE */, 0x40 /* PAGE_EXECUTE_READWRITE */);
    } else {
        core::arch::asm!("syscall", in("rax") 9, in("rdi") 0, in("rsi") size,
            in("rdx") 0x7 /* protect */, in("r10") 0x22 /* flags */,
            in("r8") -1 /* fd */, in("r9") 0 /* offset */,
            lateout("rax") ret, out("rcx") _, out("r11") _);
    }
    (if ret == 0 || ret == usize::MAX { 0 } else { ret }) as *mut u8
}

static STUB_BASE85: [u8; $$$$stub_base85_len$$$$] = *b$$$$stub_base85$$$$;
static mut BINARY_BASE85: [u8; $$$$binary_base85_len$$$$] = *b$$$$binary_base85$$$$;

#[no_mangle]
pub unsafe fn _start() -> ! {
    core::arch::asm!(
        ".quad 0xb0ec8148f0e48348,0x70248c8d48000001,0xc03101894c000001,0x48c0950f01f88349",
        ".quad 0x481049894c084189,0x4820798948185189,0x4c3051894c287189,0x2024848d48385989",
        ".quad 0xc748000001,0x840c7480000,0x1040c7480000,0x1840c7480000",
        ".quad 0x2040c7480000,0x2840c7480000,0x3040c7480000,0x3840c7480000",
        ".quad 0x89484060894c0000,0xac058d484848,0xdb3120244c8d4800,0xff111c881814b60f",
        ".quad 0xb9f27255fb83c3,0x8949d4ff41000010,0x894c20244c8d48c4,0x29e8e7894cee",
        ".quad 0xe8f78948f6894c00,0x248c8d480000001e,0x4df2894c00000120,0xb941f889",
        ".quad 0x4100000100c48148,0xf00000055bbd4ff,0xf45745df88306b6,0xe3f71104b60f16b6",
        ".quad 0x1114b60f0156b60f,0x256b60fe3f7d001,0xe3f7d0011114b60f,0x1114b60f0356b60f",
        ".quad 0x456b60fe3f7d001,0xc80fd0011114b60f,0x834805c683480789,0x323130c3b3eb04c7",
        ".quad 0x4139383736353433,0x4948474645444342,0x51504f4e4d4c4b4a,0x5958575655545352",
        ".quad 0x676665646362615a,0x6f6e6d6c6b6a6968,0x7776757473727170,0x26252423217a7978",
        ".quad 0x3d3c3b2d2b2a2928,0x7c7b605f5e403f3e,32381",
        in("r9") $$$$leading_unused_bytes$$$$,
        in("rdx") $$$$pe_image_base$$$$,
        in("rdi") $$$$pe_off_reloc$$$$,
        in("rsi") $$$$pe_size_reloc$$$$,
        in("r15") $$$$entrypoint_offset$$$$,
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