#![feature(alloc_error_handler)]
#![no_builtins]
#![no_std]
#![no_main]
extern crate alloc;

use core::arch::asm;
mod allocator;
mod io;
mod collections;
mod sorts;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[no_mangle]
#[link_section = ".init"]
fn _start() {
    unsafe {
        asm!("and rsp, 0xFFFFFFFFFFFFFFF0");
    }
    // 여기에 코드 입력...
    // 아래는 예시 코드입니다
    let mut reader = io::Reader::<{ 1 << 15 }>::new();
    let mut writer = io::Writer::<{ 1 << 15 }>::new();
    let a = reader.next_uint();
    let b = reader.next_uint();
    writer.write_uint(a + b);
    writer.flush();
    // 여기까지 예시 코드입니다
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
