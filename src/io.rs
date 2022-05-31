use core::arch::asm;
use core::mem::MaybeUninit;

pub struct Reader<const N: usize>(pub [u8; N], pub usize, pub usize);
pub struct Writer<const N: usize>(pub [u8; N], pub usize);

impl<const N: usize> Writer<N> {
    pub fn new() -> Self {
        Self(unsafe { MaybeUninit::uninit().assume_init() }, 0)
    }
    #[inline(always)]
    pub fn write(&mut self, buf: &[u8]) {
        if self.1 + buf.len() > N {
            self.flush();
        }
        for i in 0..buf.len() {
            self.0[self.1 + i] = buf[i];
        }
        self.1 += buf.len();
    }
    #[inline(always)]
    pub fn flush(&mut self) {
        unsafe {
            asm!("syscall", in("rax") 1, in("rdi") 1, in("rsi") self.0.as_ptr(), in("rdx") self.1, out("rcx") _, out("r11") _, lateout("rax") _);
        }
        self.1 = 0;
    }
    #[inline(always)]
    pub fn write_int(&mut self, i: i32) {
        if i < 0 {
            self.write(b"-");
        }
        self.write_uint(i.abs_diff(0) as usize);
    }
    #[inline(always)]
    pub fn write_long(&mut self, i: i64) {
        if i < 0 {
            self.write(b"-");
        }
        self.write_uint(i.abs_diff(0) as usize);
    }
    #[inline(always)]
    pub fn write_uint(&mut self, mut i: usize) {
        let mut buf: [u8; 20] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut offset = 19;
        buf[offset] = b'0' + (i % 10) as u8;
        i /= 10;
        while i > 0 {
            offset -= 1;
            buf[offset] = b'0' + (i % 10) as u8;
            i /= 10;
        }
        self.write(&buf[offset..]);
    }
}

impl<const N: usize> Reader<N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(unsafe { MaybeUninit::uninit().assume_init() }, 0, 0)
    }
    #[inline(always)]
    pub fn fill(&mut self) {
        let out = self.0.as_ptr();
        unsafe {
            asm!("syscall", in("rax") 0, in("rdi") 0, in("rsi") out, in("rdx") N, out("rcx") _, out("r11") _, lateout("rax") self.1);
        }
        self.2 = 0;
    }
    #[inline(always)]
    pub fn next_long(&mut self) -> i64 {
        if self.2 >= self.1 {
            self.fill();
        }
        if *unsafe { self.0.get_unchecked(self.2) } == b'-' {
            self.2 += 1;
            -(self.next_uint() as i64)
        } else {
            self.next_uint() as i64
        }
    }
    #[inline(always)]
    pub fn next_int(&mut self) -> i32 {
        if self.2 >= self.1 {
            self.fill();
        }
        if *unsafe { self.0.get_unchecked(self.2) } == b'-' {
            self.2 += 1;
            -(self.next_uint() as i32)
        } else {
            self.next_uint() as i32
        }
    }
    #[inline(always)]
    pub fn next_uint(&mut self) -> usize {
        let mut n = 0;
        loop {
            if self.2 >= self.1 {
                self.fill();
            }
            let b = *unsafe { self.0.get_unchecked(self.2) };
            self.2 += 1;
            if b > 32 {
                n *= 10;
                n += b as usize & 0x0F;
            } else {
                break;
            }
        }
        n
    }
    #[inline(always)]
    pub fn skip_white(&mut self) {
        loop {
            if self.2 >= self.1 {
                self.fill();
            }
            if *unsafe { self.0.get_unchecked(self.2) } <= 32 {
                self.2 += 1;
            } else {
                break;
            }
        }
    }
    #[inline(always)]
    pub fn next_word(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;
        loop {
            if self.2 >= self.1 {
                self.fill();
            }
            let b = *unsafe { self.0.get_unchecked(self.2) };
            self.2 += 1;
            if b <= 32 {
                break i;
            } else {
                buf[i] = b;
                i += 1;
            }
        }
    }
    #[inline(always)]
    pub fn next_line(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;
        loop {
            if self.2 >= self.1 {
                self.fill();
            }
            let b = *unsafe { self.0.get_unchecked(self.2) };
            self.2 += 1;
            if b == b'\n' {
                break i;
            } else {
                buf[i] = b;
                i += 1;
            }
        }
    }
}
