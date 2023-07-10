use core::arch::asm;

use crate::solution;
use basm::allocator;
use basm::services;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;


#[cfg(target_arch = "x86_64")]
#[no_mangle]
#[naked]
#[link_section = ".init"]
unsafe extern "sysv64" fn _start() -> ! {
    asm!(
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "mov    r12, rdi",
        "mov    rdi, QWORD PTR [rdi + 0]",
        "lea    rsi, [rip + _DYNAMIC]",
        "call   {0}",
        "mov    rdi, r12",
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "call   {1}",
        sym basm::platform::amd64::relocate, sym _start_rust, options(noreturn)
    );
}

#[cfg(all(not(target_arch = "x86_64")))]
compile_error!("The target architecture is not supported.");


extern "C" fn _start_rust(service_functions: usize) -> ! {
    services::init(service_functions);
    solution::main();
    services::exit(0)
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