use core::arch::asm;

use crate::solution;
use basm::allocator;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[no_mangle]
#[link_section = ".init"]
fn _start() {
    unsafe {
        asm!("and rsp, 0xFFFFFFFFFFFFFFF0");
    }
    solution::main();
    unsafe {
        asm!("xor eax, eax", "mov al, 231", "syscall", in("rax") 231, in("rdi") 0);
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

#[cfg(feature = "no-probe")]
#[no_mangle]
fn __rust_probestack() {}
