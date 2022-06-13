#![cfg_attr(not(test), feature(alloc_error_handler))]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(once_cell)]
#![cfg_attr(not(test), no_builtins)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

#[cfg(not(test))]
use core::{arch::asm, cmp::Ordering};
#[cfg(not(test))]
mod allocator;
#[allow(dead_code)]
mod io;
#[cfg(not(test))]
mod solution;
#[allow(dead_code)]
mod sorts;
#[allow(dead_code)]
mod syscall;

#[cfg(not(test))]
#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;

#[cfg(not(test))]
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

#[cfg(not(test))]
#[cfg(feature = "no-probe")]
#[no_mangle]
fn __rust_probestack() {}

#[cfg(not(test))]
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

#[cfg(not(test))]
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

#[cfg(not(test))]
#[no_mangle]
unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut p = s;
    for _ in 0..n {
        *p = c as u8;
        p = p.offset(1);
    }
    s
}

#[cfg(not(test))]
#[no_mangle]
unsafe extern "C" fn memcmp(mut s1: *const u8, mut s2: *const u8, n: usize) -> i32 {
    for _ in 0..n {
        match (*s1).cmp(&*s2) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            _ => {
                s1 = s1.offset(1);
                s2 = s2.offset(1);
            }
        }
    }
    0
}
