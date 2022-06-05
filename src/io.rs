use core::arch::asm;
use core::mem::MaybeUninit;

pub struct Reader<const N: usize>(pub [MaybeUninit<u8>; N], pub usize, pub usize);
pub struct Writer<const N: usize>(pub [MaybeUninit<u8>; N], pub usize);

impl<const N: usize> Writer<N> {
    pub fn new() -> Self {
        Self(MaybeUninit::uninit_array(), 0)
    }
    #[inline(always)]
    pub fn write(&mut self, buf: &[u8]) {
        if self.1 + buf.len() > N {
            self.flush();
        }
        for &b in buf {
            self.0[self.1].write(b);
            self.1 += 1;
        }
    }
    #[inline(always)]
    pub fn flush(&mut self) {
        unsafe {
            asm!("syscall", in("rax") 1, in("rdi") 1, in("rsi") self.0.as_ptr(), in("rdx") self.1, out("rcx") _, out("r11") _, lateout("rax") _);
        }
        self.1 = 0;
    }
}

impl<const N: usize> Drop for Writer<N> {
    fn drop(&mut self) {
        self.flush();
    }
}

impl<const N: usize> Reader<N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(MaybeUninit::uninit_array(), 0, 0)
    }
    #[inline(always)]
    fn peek(&mut self) -> u8 {
        if self.2 >= self.1 {
            self.fill();
        }
        unsafe { self.0.get_unchecked(self.2).assume_init_read() }
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
        if self.peek() == b'-' {
            self.2 += 1;
            -(self.next_uint() as i64)
        } else {
            self.next_uint() as i64
        }
    }
    #[inline(always)]
    pub fn next_int(&mut self) -> i32 {
        if self.peek() == b'-' {
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
            let b = self.peek();
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
            if self.peek() <= 32 {
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
            let b = self.peek();
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
            let b = self.peek();
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

// itoa implementation
//
// Copyright (C) 2014 Milo Yip
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

const DIGITS_LUT: [u8; 200] =
    *b"00010203040506070809\
       10111213141516171819\
       20212223242526272829\
       30313233343536373839\
       40414243444546474849\
       50515253545556575859\
       60616263646566676869\
       70717273747576777879\
       80818283848586878889\
       90919293949596979899";

impl<const N: usize> Writer<N> {
    #[inline(always)]
    pub fn write_u8(&mut self, v: u8) {
        let mut buf: [MaybeUninit<u8>; 3] = MaybeUninit::uninit_array();
        let mut offset = 0;
        let d1 = (v / 100) as u8;
        let d2 = ((v % 100) << 1) as usize;
        if v >= 100 {
            buf[offset].write(d1 + b'0');
            offset += 1;
        }
        if v >= 10 {
            buf[offset].write(DIGITS_LUT[d2]);
            offset += 1;
        }
        buf[offset].write(DIGITS_LUT[d2 + 1]);
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[..=offset]) });
    }

    #[inline(always)]
    pub fn write_u16(&mut self, v: u16) {
        let mut buf: [MaybeUninit<u8>; 5] = MaybeUninit::uninit_array();
        let mut offset = 0;
        let d1 = (v / 10000) as u8;
        let c = v % 10000;
        let d2 = ((c / 100) << 1) as usize;
        let d3 = ((c % 100) << 1) as usize;
        if v >= 10000 {
            buf[offset].write(d1 + b'0');
            offset += 1;
        }
        if v >= 1000 {
            buf[offset].write(DIGITS_LUT[d2]);
            offset += 1;
        }
        if v >= 100 {
            buf[offset].write(DIGITS_LUT[d2 + 1]);
            offset += 1;
        }
        if v >= 10 {
            buf[offset].write(DIGITS_LUT[d3]);
            offset += 1;
        }
        buf[offset].write(DIGITS_LUT[d3 + 1]);
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[..=offset]) });
    }

    #[inline(always)]
    pub fn write_u32(&mut self, v: u32) {
        let mut buf: [MaybeUninit<u8>; 10] = MaybeUninit::uninit_array();
        let mut offset = 0;
        if v < 10000 {
            let d1 = ((v / 100) << 1) as usize;
            let d2 = ((v % 100) << 1) as usize;
            if v >= 1000 {
                buf[offset].write(DIGITS_LUT[d1]);
                offset += 1;
            }
            if v >= 100 {
                buf[offset].write(DIGITS_LUT[d1 + 1]);
                offset += 1;
            }
            if v >= 10 {
                buf[offset].write(DIGITS_LUT[d2]);
                offset += 1;
            }
            buf[offset].write(DIGITS_LUT[d2 + 1]);
        } else if v < 100000000 {
            let b = v / 10000;
            let c = v % 10000;
            let d1 = ((b / 100) << 1) as usize;
            let d2 = ((b % 100) << 1) as usize;
            let d3 = ((c / 100) << 1) as usize;
            let d4 = ((c % 100) << 1) as usize;
            if v >= 10000000 {
                buf[offset].write(DIGITS_LUT[d1]);
                offset += 1;
            }
            if v >= 1000000 {
                buf[offset].write(DIGITS_LUT[d1 + 1]);
                offset += 1;
            }
            if v >= 100000 {
                buf[offset].write(DIGITS_LUT[d2]);
                offset += 1;
            }
            buf[offset].write(DIGITS_LUT[d2 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4 + 1]);
        } else {
            let a = v / 100000000;
            let v = v % 100000000;
            if a >= 10 {
                let a = (a << 1) as usize;
                buf[offset].write(DIGITS_LUT[a]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[a + 1]);
                offset += 1;
            } else {
                buf[offset].write(a as u8 + b'0');
                offset += 1;
            }
            let b = v / 10000;
            let c = v % 10000;
            let d1 = ((b / 100) << 1) as usize;
            let d2 = ((b % 100) << 1) as usize;
            let d3 = ((c / 100) << 1) as usize;
            let d4 = ((c % 100) << 1) as usize;
            buf[offset].write(DIGITS_LUT[d1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d1 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d2]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d2 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4 + 1]);
        }
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[..=offset]) });
    }
    
    #[inline(always)]
    pub fn write_u64(&mut self, v: u64) {
        let mut buf: [MaybeUninit<u8>; 20] = MaybeUninit::uninit_array();
        let mut offset = 0;
        if v < 100000000 {
            let v = v as u32;
            if v < 10000 {
                let d1 = ((v / 100) << 1) as usize;
                let d2 = ((v % 100) << 1) as usize;
                if v >= 1000 {
                    buf[offset].write(DIGITS_LUT[d1]);
                    offset += 1;
                }
                if v >= 100 {
                    buf[offset].write(DIGITS_LUT[d1 + 1]);
                    offset += 1;
                }
                if v >= 10 {
                    buf[offset].write(DIGITS_LUT[d2]);
                    offset += 1;
                }
                buf[offset].write(DIGITS_LUT[d2 + 1]);
            } else {
            let b = v / 10000;
            let c = v % 10000;
            let d1 = ((b / 100) << 1) as usize;
            let d2 = ((b % 100) << 1) as usize;
            let d3 = ((c / 100) << 1) as usize;
            let d4 = ((c % 100) << 1) as usize;
            if v >= 10000000 {
                buf[offset].write(DIGITS_LUT[d1]);
                offset += 1;
            }
            if v >= 1000000 {
                buf[offset].write(DIGITS_LUT[d1 + 1]);
                offset += 1;
            }
            if v >= 100000 {
                buf[offset].write(DIGITS_LUT[d2]);
                offset += 1;
            }
            buf[offset].write(DIGITS_LUT[d2 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4 + 1]);
            }
        } else if v < 10000000000000000 {
            let v0 = (v / 100000000) as u32;
            let v1 = (v % 100000000) as u32;
            let b0 = v0 / 10000;
            let c0 = v0 % 10000;
            let d1 = ((b0 / 100) << 1) as usize;
            let d2 = ((b0 % 100) << 1) as usize;
            let d3 = ((c0 / 100) << 1) as usize;
            let d4 = ((c0 % 100) << 1) as usize;
            let b1 = v1 / 10000;
            let c1 = v1 % 10000;
            let d5 = ((b1 / 100) << 1) as usize;
            let d6 = ((b1 % 100) << 1) as usize;
            let d7 = ((c1 / 100) << 1) as usize;
            let d8 = ((c1 % 100) << 1) as usize;
            if v >= 1000000000000000 {
                buf[offset].write(DIGITS_LUT[d1]);
                offset += 1;
            }
            if v >= 100000000000000 {
                buf[offset].write(DIGITS_LUT[d1 + 1]);
                offset += 1;
            }
            if v >= 10000000000000 {
                buf[offset].write(DIGITS_LUT[d2]);
                offset += 1;
            }
            if v >= 1000000000000 {
                buf[offset].write(DIGITS_LUT[d2 + 1]);
                offset += 1;
            }
            if v >= 100000000000 {
                buf[offset].write(DIGITS_LUT[d3]);
                offset += 1;
            }
            if v >= 10000000000 {
                buf[offset].write(DIGITS_LUT[d3 + 1]);
                offset += 1;
            }
            if v >= 1000000000 {
                buf[offset].write(DIGITS_LUT[d4]);
                offset += 1;
            }
            if v >= 100000000 {
                buf[offset].write(DIGITS_LUT[d4 + 1]);
                offset += 1;
            }
            buf[offset].write(DIGITS_LUT[d5]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d5 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d6]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d6 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d7]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d7 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d8]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d8 + 1]);
        } else {
            let a = (v / 10000000000000000) as u32;
            let v = (v % 10000000000000000) as u32;
            if a < 10 {
                buf[offset].write(a as u8 + b'0');
                offset += 1;
            } else if a < 100 {
                let i = (a << 1) as usize;
                buf[offset].write(DIGITS_LUT[i]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[i + 1]);
                offset += 1;
            } else if a < 1000 {
                buf[offset].write((a / 100) as u8 + b'0');
                offset += 1;
                let i = ((a % 100) << 1) as usize;
                buf[offset].write(DIGITS_LUT[i]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[i + 1]);
                offset += 1;
            } else {
                let i = ((a / 100) << 1) as usize;
                let j = ((a % 100) << 1) as usize;
                buf[offset].write(DIGITS_LUT[i]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[i + 1]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[j]);
                offset += 1;
                buf[offset].write(DIGITS_LUT[j + 1]);
                offset += 1;
            }
            let v0 = (v / 100000000) as u32;
            let v1 = (v % 100000000) as u32;
            let b0 = v0 / 10000;
            let c0 = v0 % 10000;
            let d1 = ((b0 / 100) << 1) as usize;
            let d2 = ((b0 % 100) << 1) as usize;
            let d3 = ((c0 / 100) << 1) as usize;
            let d4 = ((c0 % 100) << 1) as usize;
            let b1 = v1 / 10000;
            let c1 = v1 % 10000;
            let d5 = ((b1 / 100) << 1) as usize;
            let d6 = ((b1 % 100) << 1) as usize;
            let d7 = ((c1 / 100) << 1) as usize;
            let d8 = ((c1 % 100) << 1) as usize;
            buf[offset].write(DIGITS_LUT[d1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d1 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d2]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d2 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d3 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d4 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d5]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d5 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d6]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d6 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d7]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d7 + 1]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d8]);
            offset += 1;
            buf[offset].write(DIGITS_LUT[d8 + 1]);
        }
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[..=offset]) });
    }

    #[inline(always)]
    pub fn write_usize(&mut self, v: usize) {
        self.write_u64(v as u64);
    }

    #[inline(always)]
    pub fn write_i8(&mut self, v: i8) {
        if v.is_negative() {
            self.write(b"-");
        }
        self.write_u8(v.abs_diff(0));
    }
    
    #[inline(always)]
    pub fn write_i16(&mut self, v: i16) {
        if v.is_negative() {
            self.write(b"-");
        }
        self.write_u16(v.abs_diff(0));
    }

    #[inline(always)]
    pub fn write_i32(&mut self, v: i32) {
        if v.is_negative() {
            self.write(b"-");
        }
        self.write_u32(v.abs_diff(0));
    }

    #[inline(always)]
    pub fn write_i64(&mut self, v: i64) {
        if v.is_negative() {
            self.write(b"-");
        }
        self.write_u64(v.abs_diff(0));
    }
    
    #[inline(always)]
    pub fn write_isize(&mut self, v: isize) {
        self.write_i64(v as i64);
    }
}
