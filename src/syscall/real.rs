use core::{arch::asm, mem::MaybeUninit};

use super::Stat;

#[inline(always)]
pub fn read(fd: u32, s: &mut [u8]) -> isize {
    let len;
    unsafe {
        asm!("syscall", in("rax") 0, in("rdi") fd, in("rsi") s.as_mut_ptr(), in("rdx") s.len(), lateout("rax") len, out("rcx") _, out("r11") _);
    }
    len
}

#[inline(always)]
pub fn write(fd: u32, s: &[u8]) -> isize {
    let len;
    unsafe {
        asm!("syscall", in("rax") 1, in("rdi") fd, in("rsi") s.as_ptr(), in("rdx") s.len(), lateout("rax") len, out("rcx") _, out("r11") _);
    }
    len
}

#[inline(always)]
pub fn mmap(
    addr: *const u8,
    len: usize,
    protect: i32,
    flags: i32,
    fd: u32,
    offset: isize,
) -> *mut u8 {
    let out;
    unsafe {
        asm!("syscall", in("rax") 9, in("rdi") addr, in("rsi") len, in("rdx") protect, in("r10") flags, in("r8") fd, in("r9") offset, lateout("rax") out, out("rcx") _, out("r11") _);
    }
    out
}

#[inline(always)]
pub fn fstat(fd: u32) -> Stat {
    let stat = MaybeUninit::uninit();
    unsafe {
        asm!("syscall", in("rax") 5, in("rdi") fd, in("rsi") stat.as_ptr(), lateout("rax") _, out("rcx") _, out("r11") _);
        stat.assume_init()
    }
}
