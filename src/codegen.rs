use core::arch::asm;

use crate::solution;
use basm::allocator;
use basm::services;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;


#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
compile_error!("The target architecture is not supported.");

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
#[no_mangle]
#[naked]
#[link_section = ".init"]
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
        sym basm::platform::amd64::relocate, sym _start_rust, options(noreturn)
    );
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
#[no_mangle]
#[naked]
#[link_section = ".init"]
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
        "lea    rcx, QWORD PTR [rip + {2}]",
        "mov    BYTE PTR [rcx], 0xC3",      // put a `ret' instruction at the beginning of the function
        "1:",
        "mov    rcx, rbx",
        "call   {1}",
        sym basm::platform::amd64_windows::relocate, sym _start_rust, sym __chkstk, options(noreturn)
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
#[link_section = ".init"]
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
        sym basm::platform::i686::relocate,
        sym _start_rust,
        sym _get_dynamic_section_offset,
        options(noreturn)
    );
}

fn _start_rust(service_functions: usize) -> ! {
    services::init(service_functions);
    solution::main();
    services::exit(0)
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