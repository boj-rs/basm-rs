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
    #[inline(always)]
    pub fn iter_int(&mut self) -> IntIterator<N> {
        IntIterator { inner: self }
    }
    #[inline(always)]
    pub fn iter_long(&mut self) -> LongIterator<N> {
        LongIterator { inner: self }
    }
    #[inline(always)]
    pub fn iter_uint(&mut self) -> UintIterator<N> {
        UintIterator { inner: self }
    }
}

pub struct IntIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for IntIterator<'a, N> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_int())
    }
}

pub struct LongIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for LongIterator<'a, N> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_long())
    }
}

pub struct UintIterator<'a, const N: usize> {
    inner: &'a mut Reader<N>,
}

impl<'a, const N: usize> Iterator for UintIterator<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next_uint())
    }
}

pub trait Print<T> {
    fn print(&mut self, x: T);
    fn println(&mut self, x: T);
}

impl<const N: usize> Print<&[u8]> for Writer<N> {
    fn print(&mut self, x: &[u8]) {
        self.write(x);
    }
    fn println(&mut self, x: &[u8]) {
        self.write(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<i32> for Writer<N> {
    fn print(&mut self, x: i32) {
        self.write_i32(x);
    }
    fn println(&mut self, x: i32) {
        self.write_i32(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<i64> for Writer<N> {
    fn print(&mut self, x: i64) {
        self.write_i64(x);
    }
    fn println(&mut self, x: i64) {
        self.write_i64(x);
        self.write(b"\n");
    }
}

impl<const N: usize> Print<usize> for Writer<N> {
    fn print(&mut self, x: usize) {
        self.write_usize(x);
    }
    fn println(&mut self, x: usize) {
        self.write_usize(x);
        self.write(b"\n");
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
    pub fn write_u8(&mut self, mut v: u8) {
        let mut buf: [MaybeUninit<u8>; 3] = MaybeUninit::uninit_array();
        buf[2].write(v % 10 + b'0');
        let mut offset = 2;
        // unrolled
        for _ in 0..2 {
            v /= 10;
            if v == 0 {
                break;
            }
            offset -= 1;
            buf[offset].write(v % 10 + b'0');
        }
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[offset..]) });
    }

    #[inline(always)]
    pub fn write_u16(&mut self, mut v: u16) {
        let mut buf: [MaybeUninit<u8>; 5] = MaybeUninit::uninit_array();
        buf[4].write((v % 10) as u8 + b'0');
        let mut offset = 4;
        // unrolled
        for _ in 0..4 {
            v /= 10;
            if v == 0 {
                break;
            }
            offset -= 1;
            buf[offset].write((v % 10) as u8 + b'0');
        }
        self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buf[offset..]) });
    }

    #[inline(always)]
    pub fn write_u32(&mut self, mut v: u32) {
        use core::arch::x86_64::{_mm_add_epi8, _mm_setzero_si128, _mm_storel_epi64, _mm_cmpistri, _SIDD_CMP_EQUAL_EACH, _SIDD_NEGATIVE_POLARITY};
        if v < 100000000 {
            let a = unsafe { Self::convert_eight(v) };
            let va = unsafe { _mm_add_epi8(_mm_packus_epi16(a, _mm_setzero_si128()), Self::FILL_ZERO) };
            let digit = unsafe { _mm_cmpistri::<{ _SIDD_CMP_EQUAL_EACH | _SIDD_NEGATIVE_POLARITY }>(va, Self::FILL_ZERO) } as u32;
            let digit = digit.min(7);
            let result = unsafe { Self::shift_digits(va, digit) };
            let buffer: [u8; 16] = unsafe { transmute(result) };
            self.write(&buffer[..8 - digit as usize]);
        } else {
            let a = v / 100000000;
            v %= 100000000;
            let mut buffer: [MaybeUninit<u8>; 16] = MaybeUninit::uninit_array();
            let mut offset = 7;
            let d1 = (a / 10) as u8 + b'0';
            let d2 = (a % 10) as u8 + b'0';
            buffer[offset].write(d2);
            if a >= 10 {
                offset -= 1;
                buffer[offset].write(d1);
            }
            let a = unsafe { Self::convert_eight(v) };
            let va = unsafe { _mm_add_epi8(_mm_packus_epi16(a, _mm_setzero_si128()), Self::FILL_ZERO) };
            unsafe { _mm_storel_epi64(buffer[8..].as_mut_ptr() as _, va) };
            self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buffer[offset..]) });
        }
    }
    
    #[inline(always)]
    pub fn write_u64(&mut self, mut v: u64) {
        use core::arch::x86_64::{_mm_add_epi8, _mm_storeu_si128, _mm_cmpistri, _SIDD_CMP_EQUAL_EACH, _SIDD_NEGATIVE_POLARITY};
        if v < 100000000 {
            self.write_u32(v as u32);
        } else if v < 10000000000000000 {
            let v0 = (v / 100000000) as u32;
            let v1 = (v % 100000000) as u32;
            let a0 = unsafe { Self::convert_eight(v0) };
            let a1 = unsafe { Self::convert_eight(v1) };
            let a = unsafe { _mm_packus_epi16(a0, a1) };
            let va = unsafe { _mm_add_epi8(a, Self::FILL_ZERO) };
            let digit = unsafe { _mm_cmpistri::<{ _SIDD_CMP_EQUAL_EACH | _SIDD_NEGATIVE_POLARITY }>(va, Self::FILL_ZERO) } as u32;
            let result = unsafe { Self::shift_digits(va, digit) };
            let buffer: [u8; 16] = unsafe { transmute(result) };
            self.write(&buffer[..16 - digit as usize]);
        } else {
            let mut a = (v / 10000000000000000) as u32;
            v %= 10000000000000000;
            let mut buffer: [MaybeUninit<u8>; 32] = MaybeUninit::uninit_array();
            let mut offset = 15;
            buffer[offset].write((a % 10) as u8 + b'0');
            for _ in 0..3 {
                a /= 10;
                if a == 0 {
                    break;
                }
                offset -= 1;
                buffer[offset].write((a % 10) as u8 + b'0');
            }
            let v0 = (v / 100000000) as u32;
            let v1 = (v % 100000000) as u32;
            let a0 = unsafe { Self::convert_eight(v0) };
            let a1 = unsafe { Self::convert_eight(v1) };
            let a = unsafe { _mm_packus_epi16(a0, a1) };
            let va = unsafe { _mm_add_epi8(a, Self::FILL_ZERO) };
            unsafe { _mm_storeu_si128(buffer[16..].as_mut_ptr() as _, va) };
            self.write(unsafe { MaybeUninit::slice_assume_init_ref(&buffer[offset..]) });
        }
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

use core::arch::x86_64::{__m128i, _mm_cvtsi32_si128, _mm_srli_epi64, _mm_mul_epu32, _mm_sub_epi32, _mm_unpacklo_epi16, _mm_slli_epi64, _mm_unpacklo_epi32, _mm_mulhi_epu16, _mm_mullo_epi16, _mm_sub_epi16, _mm_srli_si128, _mm_packus_epi16};
use core::mem::transmute;

impl<const N: usize> Writer<N> {
    const DIV_10000: __m128i = unsafe { transmute([0xd1b71759u32; 4]) };
    const MUL_10000: __m128i = unsafe { transmute([10000u32; 4]) };
    const DIV_POWERS: __m128i = unsafe { transmute([8389u16, 5243, 13108, 32768, 8389, 5243, 13108, 32768]) };
    const SHIFT_POWERS: __m128i = unsafe {
        transmute([
            1u16 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << 15,
            1 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << 15,
        ])
    };
    const FILL_10: __m128i = unsafe { transmute([10u16; 8]) };
    const FILL_ZERO: __m128i = unsafe { transmute([b'0'; 16]) };

    #[inline(always)]
    unsafe fn convert_eight(v: u32) -> __m128i {
        let abcdefgh = _mm_cvtsi32_si128(v as i32);
        let abcd = _mm_srli_epi64(_mm_mul_epu32(abcdefgh, Self::DIV_10000), 45);
        let efgh = _mm_sub_epi32(abcdefgh, _mm_mul_epu32(abcd, Self::MUL_10000));
        let v1 = _mm_unpacklo_epi16(abcd, efgh);
        let v1a = _mm_slli_epi64(v1, 2);
        let v2a = _mm_unpacklo_epi16(v1a, v1a);
        let v2 = _mm_unpacklo_epi32(v2a, v2a);
        let v3 = _mm_mulhi_epu16(v2, Self::DIV_POWERS);
        let v4 = _mm_mulhi_epu16(v3, Self::SHIFT_POWERS);
        let v5 = _mm_mullo_epi16(v4, Self::FILL_10);
        let v6 = _mm_slli_epi64(v5, 16);
        let v7 = _mm_sub_epi16(v4, v6);
        v7
    }
    
    #[inline(always)]
    unsafe fn shift_digits(a: __m128i, digit: u32) -> __m128i {
        match digit {
            0 => a,
            1 => _mm_srli_si128(a, 1),
            2 => _mm_srli_si128(a, 2),
            3 => _mm_srli_si128(a, 3),
            4 => _mm_srli_si128(a, 4),
            5 => _mm_srli_si128(a, 5),
            6 => _mm_srli_si128(a, 6),
            7 => _mm_srli_si128(a, 7),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}
