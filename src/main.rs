#![feature(alloc_error_handler)]
#![no_builtins]
#![no_std]
#![no_main]
extern crate alloc;

use core::arch::asm;
mod allocator;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[no_mangle]
#[link_section = ".init"]
fn _start() {
    unsafe {
        asm!("and rsp, 0xFFFFFFFFFFFFFFF0");
    }
    // 여기에 코드 입력...
    unsafe {
        asm!("syscall", in("rax") 231, in("rdi") 0);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[alloc_error_handler]
fn alloc_fail(_: core::alloc::Layout) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
