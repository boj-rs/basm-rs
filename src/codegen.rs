use core::arch::asm;

use crate::solution;
use basm::platform;
use basm::platform::{allocator, loader};

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;


#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
compile_error!("The target architecture is not supported.");

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
#[no_mangle]
#[naked]
unsafe extern "win64" fn _start() -> ! {
    // AMD64 System V ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    asm!(
        "nop",
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "mov    r12, rcx",
        "mov    rdi, QWORD PTR [rcx + 0]",
        "lea    rsi, [rip + _DYNAMIC]",
        "call   {0}",
        "mov    rdi, r12",
        "call   {1}",
        sym loader::amd64_elf::relocate, sym _start_rust, options(noreturn)
    );
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
#[no_mangle]
#[naked]
unsafe extern "win64" fn _start() -> ! {
    // Microsoft x64 ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    // In addition, we need to provide a `shadow space' of 32 bytes
    asm!(
        "nop",
        "and    rsp, 0xFFFFFFFFFFFFFFE0",
        "sub    rsp, 32",
        "mov    rbx, rcx", // save rcx as rbx is non-volatile (callee-saved)
        "mov    rax, QWORD PTR [rbx + 72]", // PLATFORM_DATA
        "mov    rdi, QWORD PTR [rax + 16]", // ImageBase
        "mov    rsi, QWORD PTR [rbx + 0]",  // Base address of current program in memory
        "mov    rdx, QWORD PTR [rax + 24]", // Offset of relocation table
        "mov    rcx, QWORD PTR [rax + 32]", // Size of relocation table
        "call   {0}",
        "mov    rax, QWORD PTR [rbx + 72]",
        "mov    rdx, QWORD PTR [rax + 8]",
        "btc    rdx, 0",
        "jnc    1f",
        // BEGIN Linux patch
        // Linux ABI requires us to actually move the stack pointer
        //   `before' accessing the yet-to-be-committed stack pages.
        // However, it is not necessary to touch all pages in between.
        // See: https://stackoverflow.com/a/46791370
        // 0:  48 29 c4                sub    rsp, rax
        // 3:  48 85 04 24             test   QWORD PTR [rsp], rax
        // 7:  48 01 c4                add    rsp, rax
        // a:  c3                      ret 
        "lea    rcx, QWORD PTR [rip + {2}]",
        "mov    DWORD PTR [rcx], 0x48C42948",
        "mov    DWORD PTR [rcx + 4], 0x48240485",
        "mov    DWORD PTR [rcx + 8], 0x90C3C401",
        // END Linux patch
        "1:",
        "mov    rcx, rbx",
        "call   {1}",
        sym loader::amd64_pe::relocate, sym _start_rust, sym __chkstk, options(noreturn)
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
        "nop",
        "mov    edi, DWORD PTR [esp + 4]",
        "and    esp, 0xFFFFFFF0",
        "call   {2}",
        "mov    ebx, DWORD PTR [edi]",
        "add    eax, ebx",
        "sub    esp, 8",
        "push   eax",
        "push   ebx",
        "call   {0}",
        "add    esp, 4",
        "push   edi",
        "call   {1}",
        sym loader::i686_elf::relocate,
        sym _start_rust,
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

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_fail(_: core::alloc::Layout) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}


#[cfg(not(test))]
#[no_mangle]
#[allow(non_snake_case)]
pub fn _Unwind_Resume() {
    unsafe { core::hint::unreachable_unchecked() }
}

#[cfg(not(test))]
#[no_mangle]
pub fn rust_eh_personality() {
    unsafe { core::hint::unreachable_unchecked() }
}