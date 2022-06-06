use core::arch::asm;

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
