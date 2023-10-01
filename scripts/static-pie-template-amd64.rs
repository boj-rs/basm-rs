// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust and get high performance out of the box. See: https://doc.rust-lang.org/book/

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
    let (mut preferred_addr, mut off) = (0, 0);
    if cfg!(debug_assertions) && (size >> 63) == 0 {
        preferred_addr = if cfg!(windows) { 0x9_2000_0000usize } else { 0x2000_0000usize };
        off = $$$$leading_unused_bytes$$$$usize;
        size += off;
    }
    size &= (1usize << 63) - 1;
    let ret;
    if cfg!(windows) {
        ret = win_api::VirtualAlloc(preferred_addr, size,
            0x00003000 /* MEM_COMMIT | MEM_RESERVE */, 0x40 /* PAGE_EXECUTE_READWRITE */);
    } else {
        core::arch::asm!("syscall", in("rax") 9, in("rdi") preferred_addr, in("rsi") size,
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
        "and    rsp, 0xFFFFFFFFFFFFFFF0",       // Align stack to 16 byte boundary
        // [rsp+368, rsp+432]: PLATFORM_DATA
        // [rsp+288, rsp+368]: SERVICE_FUNCTIONS
        // [rsp+ 32, rsp+288]: digittobin
        // [rsp+  0, rsp+ 32]: (shadow space for win64 calling convention)
        "sub    rsp, 432",
        // Initialize base85 decoder buffer
        "lea    rax, [rip + 6f]",               // rax = b85
        "lea    rcx, QWORD PTR [rsp+ 32]",      // rcx = digittobin
        "xor    ebx, ebx",
        "2:",
        "movzx  edx, BYTE PTR [rax+rbx]",       // Upper 32bit of rdx automatically gets zeroed
        "mov    BYTE PTR [rcx+rdx], bl",
        "inc    ebx",
        "cmp    ebx, 85",
        "jb     2b",
        // PLATFORM_DATA
        "lea    rcx, QWORD PTR [rsp+368]",      // rcx = PLATFORM_DATA table
        "mov    QWORD PTR [rcx+  0], r8",       // env_id
        "mov    QWORD PTR [rcx+  8], r9",       // env_flags
        "mov    QWORD PTR [rcx+ 16], $$$$leading_unused_bytes$$$$", // leading_unused_bytes
        "mov    QWORD PTR [rcx+ 24], $$$$pe_image_base$$$$",        // pe_image_base
        "mov    QWORD PTR [rcx+ 32], $$$$pe_off_reloc$$$$",         // pe_off_reloc
        "mov    QWORD PTR [rcx+ 40], $$$$pe_size_reloc$$$$",        // pe_size_reloc
        "mov    QWORD PTR [rcx+ 48], r10",      // win_GetModuleHandleW
        "mov    QWORD PTR [rcx+ 56], r11",      // win_GetProcAddress
        // SERVICE_FUNCTIONS
        "lea    rax, QWORD PTR [rsp+288]",      // rax = SERVICE_FUNCTIONS table
        "mov    QWORD PTR [rax+  0], 0",        // ptr_imagebase
        "mov    QWORD PTR [rax+  8], 0",        // ptr_alloc
        "mov    QWORD PTR [rax+ 16], 0",        // ptr_alloc_zeroed
        "mov    QWORD PTR [rax+ 24], 0",        // ptr_dealloc
        "mov    QWORD PTR [rax+ 32], 0",        // ptr_realloc
        "mov    QWORD PTR [rax+ 40], 0",        // ptr_exit
        "mov    QWORD PTR [rax+ 48], 0",        // ptr_read_stdio
        "mov    QWORD PTR [rax+ 56], 0",        // ptr_write_stdio
        "mov    QWORD PTR [rax+ 64], r12",      // ptr_alloc_rwx
        "mov    QWORD PTR [rax+ 72], rcx",      // ptr_platform
        // Allocate memory for stub
        "movabs rcx, 0x8000000000001000",
        "call   r12",
        "mov    r15, rax",                      // r15 = stub memory
        // Decode stub (rsi -> rdi; rcx = digittobin)
        "lea    rcx, QWORD PTR [rsp+ 32]",      // rcx = digittobin
        "mov    rsi, r13",                      // rsi = STUB_BASE85
        "mov    rdi, r15",                      // rdi = stub memory
        "call   3f",
        // Decode binary (rsi -> rdi; rcx = digittobin)
        "mov    rsi, r14",                      // rsi = BINARY_BASE85
        "mov    rdi, rsi",                      // rdi = BINARY_BASE85 (in-place decoding)
        "call   3f",
        // Call stub
        "lea    rcx, QWORD PTR [rsp+288]",      // rcx = SERVICE_FUNCTIONS table
        "mov    rdx, r14",                      // rdx = LZMA-compressed binary
        "mov    r8, $$$$entrypoint_offset$$$$", // r8  = Entrypoint offset
        "mov    r9, 0",                         // r9  = 1 if debugging is enabled, otherwise 0
        "add    rsp, 256",                      // Discard digittobin
        "call   r15",
        // Base85 decoder
        "3:",
        "mov    ebx, 85",
        "4:",
        "movzx  eax, BYTE PTR [rsi]",
        "cmp    eax, 93",                       // 93 = 0x5D = b']' denotes end of base85 stream
        "je     5f",
        "movzx  edx, BYTE PTR [rsi+  0]",
        "movzx  eax, BYTE PTR [rcx+rdx]",
        "mul    ebx",
        "movzx  edx, BYTE PTR [rsi+  1]",
        "movzx  edx, BYTE PTR [rcx+rdx]",
        "add    eax, edx",
        "mul    ebx",
        "movzx  edx, BYTE PTR [rsi+  2]",
        "movzx  edx, BYTE PTR [rcx+rdx]",
        "add    eax, edx",
        "mul    ebx",
        "movzx  edx, BYTE PTR [rsi+  3]",
        "movzx  edx, BYTE PTR [rcx+rdx]",
        "add    eax, edx",
        "mul    ebx",
        "movzx  edx, BYTE PTR [rsi+  4]",
        "movzx  edx, BYTE PTR [rcx+rdx]",
        "add    eax, edx",
        "bswap  eax",
        "mov    DWORD PTR [rdi], eax",
        "add    rsi, 5",
        "add    rdi, 4",
        "jmp    4b",
        "5:",
        "ret",
        // b85 table
        "6:",
        ".quad  0x3736353433323130",
        ".quad  0x4645444342413938",
        ".quad  0x4E4D4C4B4A494847",
        ".quad  0x565554535251504F",
        ".quad  0x646362615A595857",
        ".quad  0x6C6B6A6968676665",
        ".quad  0x74737271706F6E6D",
        ".quad  0x23217A7978777675",
        ".quad  0x2D2B2A2928262524",
        ".quad  0x5F5E403F3E3D3C3B",
        ".quad  0x0000007E7D7C7B60",
        in("r8") if cfg!(windows) { 1 } else { 2 }, // Operating system ID
        in("r9") if cfg!(windows) { 0 } else { 1 }, // Enable ENV_FLAGS_LINUX_STYLE_CHKSTK outside Windows
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