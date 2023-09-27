#![cfg(not(test))]

use core::arch::asm;

use crate::solution;
use basm::platform;
use basm::platform::{allocator, loader};

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

/* We need to support multiple scenarios.
 *   1) Architectures: x86, x86-64
 *   2) Platforms for build: Windows, Linux
 *   3) Platforms on which the binary can run: Windows, Linux
 *   4) Running without the loader, running with the loader
 * This is the reason why the code is complicated.
 *
 * For 1), we implement separate versions of assembly routines.
 * For 2), we handle relocations for PE (Windows) and ELF (Linux).
 *   Also, some LLVM platform bindings that are missing on no-std builds
 *   are included when compiling on Windows. This includes __chkstk.
 * For 3), we implement a platform-abstraction layer (PAL).
 *   Also, we disable __chkstk if Windows-compiled binaries run on Linux.
 * For 4), we build the binary to run without the loader.
 *   When running without the loader, the binary will fabricate a dummy
 *     SERVICE_FUNCTIONS and PLATFORM_DATA table at the beginning of the
 *     EntryPoint (_start).
 *   When running with the loader, the loader patches the beginning of
 *     the EntryPoint (_start) to override the platform configuration data.
 *
 * When running without the loader, the relocations are handled differently.
 *   For Windows, the Windows kernel will handle relocations for us,
 *     so it is not necessary to consider them. However, we must link against
 *     the two OS functions: GetModuleHandleW and GetProcAddress. They cannot
 *     be found at runtime, unless we adopt Windows internals-dependent hacks
 *     employed in shellcodes.
 *   For Linux, we still need to handle relocations by ourselves. We need to
 *     identify the image base address and the dynamic table address. Contrary
 *     to Windows, Linux kernel ABI uses system calls, whose use don't require
 *     linking against system libraries. However, do note that in order to
 *     process relocations we temporarily need to mark the memory segments as
 *     writable. It will probably suffice to mark them as RWX through mprotect.
 */

#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
compile_error!("The target architecture is not supported.");

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
#[no_mangle]
#[naked]
unsafe extern "win64" fn _start() -> ! {
    // AMD64 System V ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    asm!(
        "xor    eax, eax",                  // rax=0 (running without loader) / rax=1 (running with loader)
        "nop",                              // padding to reserve space so that loader can patch prologue
        "nop",
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "test   rax, rax",
        "jnz    1f",
        "sub    rsp, 144",                  // 144 = 16*9 -> stack alignment preserved
        "lea    rcx, [rsp]",                // rcx = SERVICE_FUNCTIONS table
        "lea    rdx, [rsp+80]",             // rdx = PLATFORM_DATA table
        "lea    rdi, [rip + __ehdr_start]",
        "mov    QWORD PTR [rcx], rdi",      // ptr_imagebase
        "mov    QWORD PTR [rcx+72], rdx",   // ptr_platform
        "mov    QWORD PTR [rdx], 2",        // env_id = 2 (ENV_ID_LINUX)
        "mov    QWORD PTR [rdx+8], 3",      // env_flags = 3 (ENV_FLAGS_LINUX_STYLE_CHKSTK | ENV_FLAGS_NATIVE)
        "1:",
        "mov    rbx, rcx", // Save SERVICE_FUNCTIONS table
        "lea    rdi, [rip + __ehdr_start]",
        "lea    rsi, [rip + _DYNAMIC]",
        "call   {0}",
        "mov    rdi, rbx",
        "call   {1}",
        sym loader::amd64_elf::relocate, sym _start_rust, options(noreturn)
    );
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
#[link(name = "kernel32")]
extern "win64" {
    fn GetModuleHandleW(lpModuleName: *const u16) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const u8) -> usize;
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
#[no_mangle]
#[naked]
unsafe extern "win64" fn _start() -> ! {
    // Microsoft x64 ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    // In addition, we need to provide a `shadow space' of 32 bytes
    asm!(
        "xor    eax, eax",                  // rax=0 (running without loader) / rax=1 (running with loader)
        "nop",                              // padding to reserve space so that loader can patch prologue
        "nop",
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "test   rax, rax",
        "jnz    1f",
        "sub    rsp, 176",                  // 176 = 144 (tables) + 32 (shadow space) : preserves stack alignment
        "lea    rcx, [rsp+32]",             // rcx = SERVICE_FUNCTIONS table
        "lea    rdx, [rsp+112]",            // rdx = PLATFORM_DATA table
        "lea    rdi, [rip + __ImageBase]",
        "mov    QWORD PTR [rcx], rdi",      // ptr_imagebase
        "mov    QWORD PTR [rcx+72], rdx",   // ptr_platform
        "mov    QWORD PTR [rdx], 1",        // env_id = 1 (ENV_ID_WINDOWS)
        "mov    QWORD PTR [rdx+8], 2",      // env_flags = 2 (ENV_FLAGS_NATIVE)
        "lea    rax, [rip+{3}]",
        "mov    QWORD PTR [rdx+48], rax",   // GetModuleHandleW
        "lea    rax, [rip+{4}]",
        "mov    QWORD PTR [rdx+56], rax",   // GetProcAddress
        "mov    rbx, rcx",
        "jmp    2f",
        "1:",
        "mov    rbx, rcx", // save rcx as rbx is non-volatile (callee-saved)
        "mov    rax, QWORD PTR [rbx + 72]", // PLATFORM_DATA
        "mov    rdi, QWORD PTR [rax + 24]", // ImageBase
        "mov    rsi, QWORD PTR [rbx + 0]",  // Base address of current program in memory
        "mov    rdx, QWORD PTR [rax + 32]", // Offset of relocation table
        "mov    rcx, QWORD PTR [rax + 40]", // Size of relocation table
        "mov    r8, QWORD PTR [rax + 16]", // Leading unused bytes
        "sub    rsi, r8",
        "add    rdx, r8",
        "call   {0}",
        "2:",
        "mov    rax, QWORD PTR [rbx + 72]",
        "mov    rdx, QWORD PTR [rax + 8]",
        "btc    rdx, 0",
        "jnc    3f",
        // BEGIN Linux patch
        // Linux ABI requires us to actually move the stack pointer
        //   `before' accessing the yet-to-be-committed stack pages.
        // However, it is not necessary to touch the pages in advance,
        //    meaning it is okay to completely *disable* this mechanism.
        // See: https://stackoverflow.com/a/46791370
        //      https://learn.microsoft.com/en-us/cpp/build/prolog-and-epilog
        // 0:  c3                      ret
        "lea    rcx, QWORD PTR [rip + {2}]",
        "mov    BYTE PTR [rcx], 0xc3",
        // END Linux patch
        "3:",
        "mov    rcx, rbx",
        "call   {1}",
        sym loader::amd64_pe::relocate,
        sym _start_rust,
        sym __chkstk,
        sym GetModuleHandleW,
        sym GetProcAddress,
        options(noreturn)
    );
}

#[cfg(target_arch = "x86")]
#[no_mangle]
#[naked]
#[link_section = ".data"]
unsafe extern "cdecl" fn _get_start_offset() -> ! {
    asm!(
        "lea    eax, [_start]",
        "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86")]
#[no_mangle]
#[naked]
#[link_section = ".data"]
unsafe extern "cdecl" fn _get_dynamic_section_offset() -> ! {
    asm!(
        "lea    eax, [_DYNAMIC]",
        "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86")]
#[no_mangle]
#[naked]
unsafe extern "cdecl" fn _start() -> ! {
    // i386 System V ABI requires ESP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    asm!(
        "xor    eax, eax",                  // eax=0 (running without loader) / eax=1 (running with loader)
        "nop",                              // padding to reserve space so that loader can patch prologue
        "nop",
        "mov    edi, DWORD PTR [esp + 4]",  // edi = SERVICE_FUNCTIONS table
        "and    esp, 0xFFFFFFF0",
        "test   eax, eax",
        "jnz    1f",
        "sub    esp, 112",                  // 112 = 16*7 -> stack alignment preserved (8 bytes wasted for alignment)
        "lea    edi, [esp]",                // edi = SERVICE_FUNCTIONS table
        "lea    edx, [esp+40]",             // edx = PLATFORM_DATA table
        "mov    DWORD PTR [edi+36], edx",   // ptr_platform
        "mov    DWORD PTR [edx], 2",        // env_id = 2 (ENV_ID_LINUX)
        "mov    DWORD PTR [edx+4], 0",      // zero upper dword
        "mov    DWORD PTR [edx+8], 3",      // env_flags = 3 (ENV_FLAGS_LINUX_STYLE_CHKSTK | ENV_FLAGS_NATIVE)
        "mov    DWORD PTR [edx+12], 0",     // zero upper dword
        "1:",
        "call   2f",
        "2:",
        "pop    ecx",                       // ecx = _start + 60 (obtained by counting the opcode size in bytes)
        "call   {2}",                       // eax = offset of _start from the image base
        "sub    ecx, eax",
        "sub    ecx, 60",                   // ecx = the in-memory image base (i.e., __ehdr_start)
        "mov    DWORD PTR [edi], ecx",      // save __ehdr_start to cover the case of launching without loader
        "call   {3}",                       // eax = offset of _DYNAMIC table from the image base
        "add    eax, ecx",                  // eax = _DYNAMIC table
        "sub    esp, 8",                    // For stack alignment
        "push   eax",
        "push   ecx",
        "call   {0}",
        "add    esp, 4",
        "push   edi",
        "call   {1}",
        sym loader::i686_elf::relocate,
        sym _start_rust,
        sym _get_start_offset,
        sym _get_dynamic_section_offset,
        options(noreturn)
    );
}

fn _start_rust(service_functions: usize) -> ! {
    platform::init(service_functions);
    solution::main();
    platform::services::exit(0)
}

#[no_mangle]
#[naked]
#[repr(align(4))]
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
unsafe extern "win64" fn __chkstk() -> ! {
    asm!(
        "push   rcx",
        "push   rax",
        "cmp    rax, 4096",
        "lea    rcx, QWORD PTR [rsp + 24]",
        "jb     1f",
        "2:",
        "sub    rcx, 4096",
        "test   QWORD PTR [rcx], rcx", // just touches the memory address; no meaning in itself
        "sub    rax, 4096",
        "cmp    rax, 4096",
        "ja     2b",
        "1:",
        "sub    rcx, rax",
        "test   QWORD PTR [rcx], rcx", // just touches the memory address; no meaning in itself
        "pop    rax",
        "pop    rcx",
        "ret",
        options(noreturn)
    );
}

#[no_mangle]
#[cfg(target_os = "windows")]
static mut _fltused: i32 = 0;

#[no_mangle]
#[cfg(target_os = "windows")]
extern "win64" fn __CxxFrameHandler3() -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[alloc_error_handler]
fn alloc_fail(_: core::alloc::Layout) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn _Unwind_Resume() {
    unsafe { core::hint::unreachable_unchecked() }
}

#[no_mangle]
pub fn rust_eh_personality() {
    unsafe { core::hint::unreachable_unchecked() }
}