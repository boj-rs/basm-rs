#![feature(alloc_error_handler)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![no_builtins]
#![no_std]
#![no_main]
extern crate alloc;

use core::arch::asm;
mod allocator;
#[allow(dead_code)]
mod io;
mod solution;
#[allow(dead_code)]
mod sorts;

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
        asm!("syscall", in("rax") 231, in("rdi") 0);
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

#[no_mangle]
unsafe extern "C" fn memcpy(dest: *mut u8, mut src: *const u8, n: usize) -> *mut u8 {
    let mut p = dest;
    for _ in 0..n {
        *p = *src;
        p = p.offset(1);
        src = src.offset(1);
    }
    dest
}

#[no_mangle]
unsafe extern "C" fn memmove(dest: *mut u8, mut src: *const u8, n: usize) -> *mut u8 {
    let mut p = dest;
    for _ in 0..n {
        *p = *src;
        p = p.offset(1);
        src = src.offset(1);
    }
    dest
}

#[no_mangle]
unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut p = s;
    for _ in 0..n {
        *p = c as u8;
        p = p.offset(1);
    }
    s
}

#[no_mangle]
unsafe extern "C" fn memcmp(mut s1: *const u8, mut s2: *const u8, n: usize) -> i32 {
    for _ in 0..n {
        if *s1 > *s2 {
            return 1;
        } else if *s1 < *s2 {
            return -1;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    0
}
