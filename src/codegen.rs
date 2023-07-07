use core::arch::asm;

use crate::solution;
use basm::allocator;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[no_mangle]
#[link_section = ".init"]
fn _start(service_functions: usize) -> ! {
    unsafe {
        asm!("and rsp, 0xFFFFFFFFFFFFFFF0", options(nomem));
        ALLOC.init(service_functions);
    }
    solution::main();
    unsafe {
        asm!("syscall", in("rax") 231, in("rdi") 0, options(nomem, noreturn));
    }
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