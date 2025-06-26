#[cfg(not(target_arch = "wasm32"))]
use core::arch::naked_asm;

use crate::platform;
#[cfg(not(any(
    target_arch = "wasm32",
    all(target_os = "macos", target_arch = "aarch64")
)))]
use crate::platform::loader;

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
 *     EntryPoint (_basm_start).
 *   When running with the loader, the loader patches the beginning of
 *     the EntryPoint (_basm_start) to override the platform configuration data.
 *
 * When running without the loader, the relocations are handled differently.
 *   For Windows, the Windows kernel will handle relocations for us,
 *     so it is not necessary to consider them. However, we must link against
 *     the two OS functions: LoadLibraryA (or GetModuleHandleW) and GetProcAddress.
 *     They cannot be found at runtime, unless we adopt Windows internals-dependent
 *     hacks employed in shellcodes.
 *   For Linux, we still need to handle relocations by ourselves. We need to
 *     identify the image base address and the dynamic table address. Contrary
 *     to Windows, Linux kernel ABI uses system calls, whose use don't require
 *     linking against system libraries. However, do note that in order to
 *     process relocations we temporarily need to mark the memory segments as
 *     writable. It will probably suffice to mark them as RWX through mprotect;
 *     we actually don't even bother to call mprotect, trusting the linker to
 *     have done a good job of marking sections needing relocations as writable.
 */

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "wasm32"
)))]
compile_error!("The target architecture is not supported.");

#[cfg(all(target_arch = "aarch64", feature = "submit"))]
compile_error!(
    "AArch64 (aarch64-apple-darwin) is only supported for local execution, not submission; use x86-64 to submit."
);

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "win64" fn _basm_start() -> ! {
    // AMD64 System V ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call` instruction.
    // However, when called as the entrypoint by the Linux OS,
    //   RSP will be 16-byte aligned AFTER `call` instruction.
    naked_asm!(
        "clc",                              // CF=0 (running without loader) / CF=1 (running with loader)
        "push   rcx",                       // short form of "sub rsp, 8"
        "jnc    2f",
        "test   rcx, rcx",
        "jnz    3f",
        "2:",
        "enter  72, 0",                     // approximately equivalent to "sub rsp, 80" with more zeros (possibly smaller code).
                                            //   8 + 80 + 16 + 8 = 112 = 16*7 -> stack alignment preserved
        "push   3",                         // env_flags = 3 (ENV_FLAGS_LINUX_STYLE_CHKSTK | ENV_FLAGS_NATIVE)
        "push   2",                         // env_id = 2 (ENV_ID_LINUX)
        "push   rsp",                       // rcx = PLATFORM_DATA table (short form of "lea rcx, [rsp]")
        "3:",
        "lea    rdi, [rip + __ehdr_start]",
        "lea    rsi, [rip + _DYNAMIC]",
        "push   rdi",
        "pop    rbx",                       // rbx = in-memory ImageBase
        "call   {0}",
        "pop    rdi",                       // rdi = PLATFORM_DATA table
        "mov    QWORD PTR [rdi + 32], rbx", // overwrite ptr_alloc_rwx with in-memory ImageBase
        "jmp    {1}",                       // _start_rust will inherit the initial call stack of this function
        sym loader::amd64_elf::relocate,
        sym _start_rust
    )
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
#[link(name = "kernel32")]
unsafe extern "win64" {
    fn LoadLibraryA(lpLibFileName: *const i8) -> usize;
    fn GetProcAddress(hModule: usize, lpProcName: *const i8) -> usize;
}

#[cfg(target_os = "windows")]
unsafe extern "sysv64" fn get_kernel32() -> usize {
    unsafe { LoadLibraryA(c"KERNEL32".as_ptr()) }
}

#[cfg(all(target_os = "windows", target_env = "gnu"))]
mod chkstk_gnu {
    unsafe extern "C" {
        pub fn ___chkstk_ms();
        pub fn ___chkstk();
    }
}
#[cfg(all(target_os = "windows", not(target_env = "gnu")))]
mod chkstk_gnu {
    pub extern "C" fn ___chkstk_ms() {}
    pub extern "C" fn ___chkstk() {}
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "win64" fn _basm_start() -> ! {
    // Microsoft x64 ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call` instruction.
    // Also, when called as the entrypoint by the Windows OS,
    //   RSP will be 16-byte aligned BEFORE `call` instruction.
    // In addition, we need to provide a `shadow space` of 32 bytes.
    naked_asm!(
        "clc",                              // CF=0 (running without loader) / CF=1 (running with loader)
        "enter  64, 0",                     // 64 = 88 - 32 (tables) + 8 (alignment)
        "mov    rbx, rcx",                  // save rcx as rbx is non-volatile (callee-saved)
        "jnc    2f",
        "test   rbx, rbx",
        "jnz    3f",                        // jump not taken if running under shorter template
        "push   rbx",                       // GetProcAddress = 0
        "push   rbx",                       // handle to kernel32 = 0
        "push   1",                         // env_flags = 1 (ENV_FLAGS_LINUX_STYLE_CHKSTK)
        "push   2",                         // env_id = 2 (ENV_ID_LINUX)
        "mov    rbx, rsp",                  // rbx = PLATFORM_DATA table
        "lea    rsp, [rsp - 40]",           // extra 8 bytes for stack alignment
        "jmp    3f",
        "2:",
        "call   {3}",
        "lea    rdi, [rip+{4}]",
        "push   rdi",                       // GetProcAddress
        "push   rax",                       // handle to kernel32
        "push   2",                         // env_flags = 2 (ENV_FLAGS_NATIVE)
        "push   1",                         // env_id = 1 (ENV_ID_WINDOWS)
        "mov    rbx, rsp",                  // rbx = PLATFORM_DATA table
        "lea    rsp, [rsp - 32]",
        "jmp    4f",
        "3:",
        "lea    rdi, [rip + __ImageBase]",  // In-memory ImageBase (cf. Preferred ImageBase is set to 0x0 by static-pie-pe2bin.py)
        "mov    esi, 0x12345678",           // [replaced by static-pie-pe2bin.py] Offset of relocation table (relative to the in-memory ImageBase)
        "mov    edx, 0x12345678",           // [replaced by static-pie-pe2bin.py] Size of relocation table (relative to the in-memory ImageBase)
        "mov    QWORD PTR [rbx + 32], rdi", // overwrite ptr_alloc_rwx with in-memory ImageBase
        "call   {0}",
        "4:",
        "bt     QWORD PTR [rbx + 8], 0",
        "jnc    5f",
        // BEGIN Linux patch
        // Linux ABI requires us to actually move the stack pointer
        //   `before' accessing the yet-to-be-committed stack pages.
        // However, it is not necessary to touch the pages in advance,
        //    meaning it is okay to completely *disable* this mechanism.
        // See: https://stackoverflow.com/a/46791370
        //      https://learn.microsoft.com/en-us/cpp/build/prolog-and-epilog
        // 0:  c3                      ret
        "mov    BYTE PTR [rip + {2}], 0xc3",
        "mov    BYTE PTR [rip + {5}], 0xc3",
        "mov    BYTE PTR [rip + {6}], 0xc3",
        // END Linux patch
        "5:",
        "mov    rcx, rbx",
        "call   {1}",
        "leave",
        "ret",
        sym loader::amd64_pe::relocate,
        sym _start_rust,
        sym __chkstk,
        sym get_kernel32,
        sym GetProcAddress,
        sym chkstk_gnu::___chkstk_ms,
        sym chkstk_gnu::___chkstk
    )
}

#[cfg(target_arch = "x86")]
extern "cdecl" fn _get_start_offset() -> usize {
    _basm_start as usize
}

#[cfg(target_arch = "x86")]
extern "cdecl" fn _get_dynamic_section_offset() -> usize {
    unsafe extern "C" {
        fn _DYNAMIC();
    }
    _DYNAMIC as usize
}

#[cfg(target_arch = "x86")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "cdecl" fn _basm_start() -> ! {
    // i386 System V ABI requires ESP to be aligned
    //   on the 16-byte boundary BEFORE `call` instruction
    naked_asm!(
        "clc",                              // CF=0 (running without loader) / CF=1 (running with loader)
        "jc     2f",
        "sub    esp, 44",                   // 44 = 40 + 4; PLATFORM_DATA ptr (4 bytes, pushed later) + PLATFORM_DATA (40 (+ 16 = 56 bytes)) + alignment (4 bytes wasted)
        "push   0",                         // zero upper dword
        "push   3",                         // env_flags = 3 (ENV_FLAGS_LINUX_STYLE_CHKSTK | ENV_FLAGS_NATIVE)
        "push   0",                         // zero upper dword
        "push   2",                         // env_id = 2 (ENV_ID_LINUX)
        "mov    edx, esp",                  // edx = PLATFORM_DATA table
        "jmp    3f",
        "2:",
        "mov    edx, DWORD PTR [esp + 4]",  // edx = PLATFORM_DATA table
        "push   ebp",
        "mov    ebp, esp",
        "and    esp, 0xFFFFFFF0",
        "sub    esp, 12",
        "3:",
        "call   4f",
        "4:",
        "pop    ebx",                       // ebx = _basm_start + 36 (obtained by counting the opcode size in bytes)
        "push   edx",                       // [esp + 0] = PLATFORM_DATA table, stack is aligned
        "call   {2}",                       // eax = offset of _basm_start from the image base
        "sub    ebx, eax",
        "sub    ebx, 36",                   // ebx = the in-memory image base (i.e., __ehdr_start)
        "call   {3}",                       // eax = offset of _DYNAMIC table from the image base
        "lea    esi, [ebx + eax]",          // esi = _DYNAMIC table
        "sub    esp, 8",                    // For stack alignment
        "push   esi",
        "push   ebx",
        "call   {0}",                       // call loader::i686_elf::relocate
        "add    esp, 16",
        "call   {1}",
        "mov    esp, ebp",
        "pop    ebp",
        "ret",
        sym loader::i686_elf::relocate,
        sym _start_rust,
        sym _get_start_offset,
        sym _get_dynamic_section_offset
    )
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn _basm_start() {
    let mut pd = platform::services::PlatformData {
        env_id: platform::services::ENV_ID_WASM,
        ..Default::default()
    };
    _start_rust(&mut pd as *mut platform::services::PlatformData as usize);
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[align(8)]
pub unsafe extern "C" fn _basm_start() -> ! {
    naked_asm!(
        "sub    sp, sp, #96",
        "mov    x0, #4",    // 4 = ENV_ID_MACOS
        "str    x0, [sp, #(8 * 0)]",
        "mov    x0, #2",    // 2 = ENV_FLAGS_NATIVE
        "str    x0, [sp, #(8 * 1)]",
        "mov    x0, sp",
        "bl     {0}",
        sym _start_rust
    )
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _basm_start() -> ! {
    naked_asm!(
        "adrp   x0, __ehdr_start",
        "add    x0, x0, #:lo12:__ehdr_start",
        "adrp   x1, _DYNAMIC",
        "add    x1, x1, #:lo12:_DYNAMIC",
        "bl     {0}",
        "sub    sp, sp, #96",
        "mov    x0, #2",    // 2 = ENV_ID_LINUX
        "str    x0, [sp, #(8 * 0)]",
        "mov    x0, #2",    // 2 = ENV_FLAGS_NATIVE
        "str    x0, [sp, #(8 * 1)]",
        "mov    x0, sp",
        "bl     {1}",
        sym loader::aarch64_elf::relocate,
        sym _start_rust
    )
}

/* We prevent inlining solution::main, since if the user allocates
 * a large amount of stack memory there, it will be zero-initialized (or probed)
 * *before* we increase the stack limits if it is inlined into _start_rust.
 * This will cause stack overflow, thus we prevent it.
 */
#[cfg_attr(not(feature = "short"), inline(never))]
fn _call_main() {
    unsafe extern "C" {
        fn _basm_main();
    }
    unsafe { _basm_main() }
}
fn _start_rust(platform_data: usize) -> i32 {
    platform::init(platform_data);
    _call_main();
    platform::try_exit();
    platform::services::get_exit_status()
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[align(4)]
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub unsafe extern "win64" fn __chkstk() -> ! {
    naked_asm!(
        "push   rcx",
        "push   rax",
        "cmp    rax, 4096",
        "lea    rcx, QWORD PTR [rsp + 24]",
        "jb     2f",
        "3:",
        "sub    rcx, 4096",
        "test   DWORD PTR [rcx], ecx", // just touches the memory address; no meaning in itself
        "sub    rax, 4096",
        "cmp    rax, 4096",
        "ja     3b",
        "2:",
        "sub    rcx, rax",
        "test   DWORD PTR [rcx], ecx", // just touches the memory address; no meaning in itself
        "pop    rax",
        "pop    rcx",
        "ret"
    )
}

pub unsafe fn print_panicinfo_and_exit(_pi: &core::panic::PanicInfo) -> ! {
    unsafe {
        use crate::platform::services::write_stdio;
        use alloc::string::ToString;
        write_stdio(2, _pi.to_string().as_bytes());
        write_stdio(2, b"\n");

        // Rust sets an exit code of 101 when the process panicked.
        // Hence, we follow that practice for maximum compatibility.
        // Reference: https://rust-cli.github.io/book/in-depth/exit-code.html
        #[cfg(all(windows, target_arch = "x86_64"))]
        {
            unsafe extern "win64" {
                fn ExitProcess(uExitCode: u32) -> !;
            }
            ExitProcess(101)
        }
        #[cfg(target_os = "linux")]
        {
            crate::platform::os::linux::syscall::exit_group(101)
        }
        #[cfg(target_os = "macos")]
        {
            crate::platform::os::macos::syscall::exit_group(101)
        }
        #[cfg(not(any(
            all(windows, target_arch = "x86_64"),
            target_os = "linux",
            target_os = "macos"
        )))]
        core::hint::unreachable_unchecked()
    }
}
