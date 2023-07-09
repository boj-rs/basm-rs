use core::arch::asm;

use crate::solution;
use basm::allocator;
use basm::services;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[no_mangle]
#[link_section = ".init"]
fn _start(service_functions: usize) -> ! {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!("and rsp, 0xFFFFFFFFFFFFFFF0", options(nomem));
        #[cfg(target_arch = "x86")]
        asm!("and esp, 0xFFFFFFF0", options(nomem));
    }
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