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

unsafe extern "win64" fn svc_alloc_rwx(mut size: usize) -> *mut u8 {
    let off = $$$$leading_unused_bytes$$$$usize;
    size += off;
    size &= (1usize << 63) - 1;
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
    (if ret == 0 || ret == usize::MAX { 0 } else { ret + off }) as *mut u8
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
        ".quad 0x89484060894c0000,0xb1058d484848,0xdb3120244c8d4800,0xff111c881814b60f",
        ".quad 0xb948f27255fb83c3,0x8000000000001000,0x8d48c48949d4ff41,0x894cee894c20244c",
        ".quad 0x894c00000029e8e7,0x1ee8f78948f6,0x120248c8d4800,0x41f8894df2894c00",
        ".quad 0xc4814800000000b9,0xbbd4ff4100000100,0x8306b60f00000055,0xf16b60f45745df8",
        ".quad 0x56b60fe3f71104b6,0xf7d0011114b60f01,0x14b60f0256b60fe3,0x56b60fe3f7d00111",
        ".quad 0xf7d0011114b60f03,0x14b60f0456b60fe3,0x480789c80fd00111,0xeb04c7834805c683",
        ".quad 0x353433323130c3b3,0x4443424139383736,0x4c4b4a4948474645,0x54535251504f4e4d",
        ".quad 0x62615a5958575655,0x6a69686766656463,0x7271706f6e6d6c6b,0x7a79787776757473",
        ".quad 0x2a29282625242321,0x403f3e3d3c3b2d2b,0x7e7d7c7b605f5e",
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